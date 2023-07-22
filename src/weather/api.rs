use chrono;
use core::panic;
use log::{info, error};
use std::{thread, time};
use ureq;
use ureq::Error;
use crate::storage;
use crate::ecobee;
use crate::weather::models;

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
    info!("Initializing weather loop. Current hvac mode is {hvac_mode}");
    loop {
        match ureq::post("https://api.weatherapi.com/v1/current.json")
        .query("key", weather_settings.api_key.as_ref().unwrap().as_str())
        .query("q", weather_settings.query.as_ref().unwrap().as_str())
        .call() {
            Ok(response) => {
                match response.into_json::<models::WeatherResponse>() {
                    Ok(resp) => {
                        let temp: Option<f64> = if weather_settings.metric.unwrap() {
                            resp.current.temp_c
                        } else { resp.current.temp_f };
                        match temp {
                            Some(t) => { 
                                let timestamp = chrono::offset::Local::now().to_rfc2822();
                                info!("Checking temp ({t}) @ {timestamp}");

                                // If the temp is above our thresholds, prioritize cooling, otherwise turn off, otherwise leave alone.
                                if weather_settings.cool_above.is_some() && t > weather_settings.cool_above.unwrap() && hvac_mode != "cool" {
                                    info!("Current temp: {t} current mode: {hvac_mode} - change @ {timestamp}");
                                    ecobee::api::refresh_tokens();
                                    ecobee::api::update_thermostats("cool");
                                    hvac_mode = "cool";
                                }
                                else if weather_settings.off_above.is_some() && t > weather_settings.off_above.unwrap() && hvac_mode != "off" {

                                }
                                
                                // If the temp is below our thresholds, prioritize heating, otherwise turn off, otherwise leave alone.
                                if weather_settings.heat_below.is_some() && t < weather_settings.heat_below.unwrap() && hvac_mode != "heat" {
                                    info!("Current temp: {t} current mode: {hvac_mode} - change @ {timestamp}");
                                    ecobee::api::refresh_tokens();
                                    ecobee::api::update_thermostats("heat");
                                    hvac_mode = "heat";
                                }
                                else if weather_settings.off_below.is_some() && t < weather_settings.off_below.unwrap() && hvac_mode != "off" {

                                }
                            },
                            _ => error!("No temperature found! Something went wrong with the temperature pulled from WeatherAPI: {temp:?}")
                        }
                    },
                    Err(e) => error!("{e:?}")
                }
            },
            Err(Error::Status(code, response)) => {
                error!("Error with request for weather: {code} \n{}", response.into_string().unwrap());
            }
            Err(e) => { error!("Transport error: {e}") }
        }
        thread::sleep(duration);
    }
}