use chrono;
use core::panic;
use log::{info, error};
use std::{thread, time};
use ureq;
use std::error::Error;
use crate::storage;
use crate::ecobee;
use crate::weather::models;

use super::models::WeatherSettings;

/// # get_temp()
/// 
/// Get the temperature using weather api.
fn get_temp(weather_settings: &WeatherSettings) -> Result<f64, Box<dyn Error>> {
    ecobee::api::refresh_tokens();
    let response = ureq::post("https://api.weatherapi.com/v1/current.json")
    .query("key", weather_settings.api_key.as_ref().unwrap().as_str())
    .query("q", weather_settings.query.as_ref().unwrap().as_str())
    .call()?;
    let json_response = response.into_json::<models::WeatherResponse>()?;            
    let temp: Option<f64> = if weather_settings.metric.unwrap() {
        json_response.current.temp_c
    } else { json_response.current.temp_f };
    match temp {
        Some(t) => Ok(t),
        _ => Err("No temperature found! Something went wrong with the temperature pulled from WeatherAPI: {temp:?}")?
    }
}

/// # check()
/// 
/// Check the weather using weather api.
/// 
/// (May differ from other sources - eg check against weather.com)
pub fn check() {
    let weather_settings = storage::load_weather_settings();
    let temp = get_temp(&weather_settings);
    match temp {
        Ok(t) => { 
            let timestamp = chrono::offset::Local::now().to_rfc2822();
            println!("Current temp is {t} as of {timestamp}");
        },
        Err(e) => error!("{:?}", e)
    }
}

/// # set_hvac()
/// 
/// Set the hvac mode, log to info the current temp and mode first, along with the change timestamp.
fn set_hvac<'a>(mode: &'a str, temp: f64, timestamp: String, hvac_mode: &'a str) -> &'a str {
    info!("Current temp: {temp} current mode: {hvac_mode} - change @ {timestamp}");
    ecobee::api::refresh_tokens();
    ecobee::api::update_thermostats(mode);
    mode
}

/// # run()
/// 
/// Run weather mode in an infinite loop (until broken by user input).
/// 
/// Apply a change if the temperature is above or below thresholds specified during weather setup, otherwise continue doing nothing.
pub fn run() {
    let weather_settings = storage::load_weather_settings();
    if weather_settings.interval.is_none() {
        panic!("Interval is not set. Run --weathersetup before proceeding.");
    }
    let duration = time::Duration::from_secs(weather_settings.interval.unwrap() * 60);
    ecobee::api::refresh_tokens();
    let binding = ecobee::api::thermostat_status(); // This will either return a consistent mode (heat, cool, off) or "inconsistent".
    let mut hvac_mode = binding.as_str();
    let initial_timestamp = chrono::offset::Local::now().to_rfc2822();
    let duration_minutes = duration.as_secs() / 60;
    println!("Initializing weather loop @ {initial_timestamp} checking every {duration_minutes} minutes.\nCurrent hvac mode is {hvac_mode}");
    loop {
        let temp = get_temp(&weather_settings);
        match temp {
            Ok(t) => { 
                let timestamp = chrono::offset::Local::now().to_rfc2822();
                info!("Checking temp ({t}) @ {timestamp}");

                // If the temp is above our thresholds, prioritize cooling, otherwise turn off, otherwise leave alone.
                if weather_settings.cool_above.is_some() && t > weather_settings.cool_above.unwrap() && hvac_mode != "cool" {
                    hvac_mode = set_hvac("cool", t, timestamp, hvac_mode);
                }
                else if weather_settings.off_above.is_some() && t > weather_settings.off_above.unwrap() && hvac_mode != "off" {
                    hvac_mode = set_hvac("off", t, timestamp, hvac_mode);
                }
                
                // If the temp is below our thresholds, prioritize heating, otherwise turn off, otherwise leave alone.
                else if weather_settings.heat_below.is_some() && t < weather_settings.heat_below.unwrap() && hvac_mode != "heat" {
                    hvac_mode = set_hvac("heat", t, timestamp, hvac_mode);
                }
                else if weather_settings.off_below.is_some() && t < weather_settings.off_below.unwrap() && hvac_mode != "off" {
                    hvac_mode = set_hvac("off", t, timestamp, hvac_mode);
                }
            },
            Err(e) => error!("{:?}", e)
        }
        thread::sleep(duration);
    }
}