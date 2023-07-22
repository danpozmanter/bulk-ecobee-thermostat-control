
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {

    pub current: WeatherResponseCurrent,
}

#[derive(Deserialize, Debug)]
pub struct WeatherResponseCurrent {

    pub temp_c: Option<f64>,
    pub temp_f: Option<f64>
}

/// # WeatherSettings
/// 
/// For storing/retrieving Weather API settings and usage settings for Weather Mode.
#[derive(Deserialize, Debug, Serialize)]
pub struct WeatherSettings {

    pub api_key: Option<String>,
    pub query: Option<String>,
    pub metric: Option<bool>,
    pub heat_below: Option<f64>,
    pub cool_above: Option<f64>,
    pub off_above: Option<f64>,
    pub off_below: Option<f64>,
    pub interval: Option<u64>
}