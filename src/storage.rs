
use log::{debug, error};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;
use std::str::FromStr;

use crate::ecobee::models::{Tokens, ThermostatMeta};
use crate::weather::models::{WeatherSettings};

static CONFIG_DIRECTORY: &str = ".bulk_ecobee_thermostat_control";
static API_FILENAME: &str = "api_key";
static TOKENS_FILENAME: &str = "api_tokens";
static THERMOSTATS_FILENAME: &str = "thermostats.yaml";
static WEATHER_FILENAME: &str = "weather.yaml";

// NOTE: Functions in this file panic on error.

/// # create_config_dir
/// 
/// Create the configuration directory if it doesn't already exist, or do nothing.
pub fn create_config_dir() {
    let dir = get_config_base_path();
    match fs::create_dir_all(dir) {
        Ok(_) => (),
        Err(e) => panic!("Error creating config directory: {}", e)
    }
}

/// # get_config_base_path
/// 
/// Return the base path for this application's configuration.
fn get_config_base_path() -> String {
    match home::home_dir() {
        Some(path) => { 
            match path.as_path().to_str() {
                Some(home_path) => format!("{}/{}", home_path, String::from_str(CONFIG_DIRECTORY).unwrap()),
                None => panic!("Error converting home directory to string.")
            }
        },
        None => panic!("Error getting your home directory."),
    }
}

/// # get_config_file_path(filename: &str) -> String
/// 
/// Get the absolute file path for the filename in the configuration directory
fn get_config_file_path(filename: &str) -> String {
    format!("{}/{}", get_config_base_path(), filename)
}


/// # load_app_key() -> String
/// 
/// Load the app/api key from local storage.
pub fn load_app_key() -> String {
    let mut file = match File::open(get_config_file_path(API_FILENAME)) {
        Ok(f) => f,
        Err(e) => { panic!("Error opening api key file: {e}"); }
    };
    let mut contents = String::new();
    
    match file.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(e) => { panic!("Error reading ecobee_api_key file contents: {e}"); }
    }
}

/// # load_thermostats() -> Vec<ThermostatMeta>
/// 
/// Load the thermostat metadata for all registered thermostats (identifier and name).
pub fn load_thermostats() -> Vec<ThermostatMeta> {
    let mut file = match File::open(get_config_file_path(THERMOSTATS_FILENAME)) {
        Ok(f) => f,
        Err(e) => { panic!("Error opening thermostats file: {e}"); }
    };
    let mut contents = String::new();
    
    let content = match file.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(e) => { panic!("Error reading thermostats contents: {e}"); }
    };

    let thermostats: Vec<ThermostatMeta> = serde_yaml::from_str(&content).unwrap();

    thermostats
}

/// # load_tokens() -> Tokens
/// 
/// Load access and refresh tokens (or just the initial access code and a blank string) from local storage.
pub fn load_tokens() -> Tokens {
    let mut file = match File::open(get_config_file_path(TOKENS_FILENAME)) {
        Ok(f) => f,
        Err(e) => { panic!("Error opening tokens file: {e}"); }
    };
    let mut contents = String::new();
    
    let content = match file.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(e) => { panic!("Error reading tokens file contents: {e}"); }
    };

    let mut csplit = content.split('\n');
    let access_token = csplit.next();
    let refresh_token = csplit.next();

    Tokens { access_token: access_token.unwrap().to_string(), refresh_token: refresh_token.unwrap().to_string()}
}

/// # load_weather_settings() -> WeatherSettings
/// 
/// Load the thermostat metadata for all registered thermostats (identifier and name).
pub fn load_weather_settings() -> WeatherSettings {
    let mut file = match File::open(get_config_file_path(WEATHER_FILENAME)) {
        Ok(f) => f,
        Err(_) => { 
            error!("Could not open weather settings file.");
            return WeatherSettings{
                api_key: None, 
                query: None, 
                metric: None,
                heat_below: None, 
                cool_above: None,
                off_above: None,
                off_below: None,
                interval: None };
        }
    };
    let mut contents = String::new();
    
    let content = match file.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(e) => { panic!("Error reading weather settings contents: {e}"); }
    };

    let weather_settings: WeatherSettings = serde_yaml::from_str(&content).unwrap();

    weather_settings
}


/// # write_api_key(api_key: String)
/// 
/// Write the api_key entered by the user into local storage.
pub fn write_api_key(api_key: String) {

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_config_file_path(API_FILENAME));
    match file {
        Ok(mut f) => {
            match write!(f, "{api_key}") {
                Ok(_) => debug!("Successfully wrote api key."),
                Err(e) => panic!("Error writing api key {:?}", e.to_string())
            }
        },
        Err(e) => panic!("Error writing api key {:?}", e.to_string())
    }
}


/// # write_thermostats(thermostats: Vec<ThermostatMeta>)
/// 
/// Write the thermostat metadata into local storage for use during updates.
pub fn write_thermostats(thermostats: Vec<ThermostatMeta>) {

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_config_file_path(THERMOSTATS_FILENAME));
    match file {
        Ok(mut f) => {
            match write!(f, "{}", serde_yaml::to_string(&thermostats).unwrap()) {
                Ok(_) => (),
                Err(e) => panic!("Error writing thermostat metadata {:?}", e.to_string())
            }
        },
        Err(e) => panic!("Error writing thermostat metadata {:?}", e.to_string())
    }
}

/// # write_tokens(access_token: String, refresh_token: String)
/// 
/// Write the updated access and refresh tokens to local storage.
pub fn write_tokens(access_token: String, refresh_token: String) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_config_file_path(TOKENS_FILENAME));
    match file {
        Ok(mut f) => {
            match write!(f, "{access_token}\n{refresh_token}") {
                Ok(_) => (),
                Err(e) => panic!("Error writing access tokens {:?}", e.to_string())
            }
        },
        Err(e) => panic!("Error writing access tokens {:?}", e.to_string())
    }
}

/// # write_weather_settings(weather_settings: WeatherSettings)
/// 
/// Write the thermostat metadata into local storage for use during updates.
pub fn write_weather_settings(weather_settings: WeatherSettings) {

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_config_file_path(WEATHER_FILENAME));
    match file {
        Ok(mut f) => {
            match write!(f, "{}", serde_yaml::to_string(&weather_settings).unwrap()) {
                Ok(_) => (),
                Err(e) => panic!("Error writing thermostat metadata {:?}", e.to_string())
            }
        },
        Err(e) => panic!("Error writing thermostat metadata {:?}", e.to_string())
    }
}