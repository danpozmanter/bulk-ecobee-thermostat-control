use std::io::{self, Write};
use std::str::FromStr;
use crate::storage;
use crate::weather::models;


/// # setup()
/// 
/// User entered settings for weather mode, including API credentials.
pub fn setup() {
    println!("Weather setup.");
    println!("Press <ENTER> to skip an entry and keep the current value.");
    println!("Press <SPACE> then <ENTER> to set an entry to empty (unset).");
    let weather_settings = storage::load_weather_settings();
    let api_key = get_value::<String>("weatherapi.com API Key", weather_settings.api_key);
    let query = get_value::<String>("query", weather_settings.query);
    let metric = get_value::<bool>("use metric?", weather_settings.metric);
    let cool_above = get_value::<f64>("cool above", weather_settings.cool_above);
    let heat_below = get_value::<f64>("heat below", weather_settings.heat_below);
    let off_above = get_value::<f64>("turn hvac off above", weather_settings.off_above);
    let off_below = get_value::<f64>("turn hvac off below", weather_settings.off_below);
    let interval =  get_value::<u64>("interval in minutes", weather_settings.interval);

    if !validate(&api_key, &query, &cool_above, &heat_below, &off_above, &off_below, &interval) {
        return
    }
    storage::write_weather_settings(models::WeatherSettings{
        api_key,
        query,
        metric,
        heat_below,
        cool_above,
        off_above,
        off_below,
        interval,
    });    
}

/// # get_value()
/// 
/// Return a string or a parsed int, float, or boolean from stdin using the provided message.
fn get_value<T: FromStr + ToString>(msg: &str, current_value: Option<T>) -> Option<T> {
    let mut entry = String::new();
    let cv = if current_value.is_some() {
        current_value.as_ref().unwrap().to_string()
    } else { "unset".to_string() };
    print!("{msg} (current_value: {})> ", cv);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entry).unwrap();
    entry.truncate(entry.len() - 1);
    if entry == " " {
        return None;
    }
    if !entry.is_empty() {
        let parsed: Result<T, <T as FromStr>::Err> = entry.parse::<T>();
        return match parsed {
            Ok(v) => Some(v),
            _ => None
        }
    }
    if cv == "unset" {
        return None;
    }
    current_value
}

/// # validate()
/// 
/// Check for conflicts and bad values.
/// 
/// api_key, interval, and query must be set.
/// 
/// cool_above and heat_below must be set.
/// 
/// mode settings must follow this guide:
/// 
/// cool_above > off_above > off_below > heat_below
/// 
/// Where each, if set, must be greater than (and not equal to) the following setting.
/// 
/// This should prevent conflicts, or turning off cooling at high temperatures, or turning off heating at low temperatures.
fn validate(
    api_key: &Option<String>, query: &Option<String>,
    cool_above: &Option<f64>, heat_below: &Option<f64>, off_above: &Option<f64>, off_below: &Option<f64>,
    interval: &Option<u64>
) -> bool {
    let mut valid = true;
    if api_key.is_none() {
        println!("ERROR: API Key unset.");
        valid = false;
    }
    if interval.is_none() {
        println!("ERROR: Interval unset.");
        valid = false;
    }
    if query.is_none() {
        println!("ERROR: Query unset.");
        valid = false;
    }
    if cool_above.is_none() {
        println!("ERROR: Cool Above unset.");
        valid = false;
    }
    if heat_below.is_none() {
        println!("ERROR: Heat Below unset.");
        valid = false;
    }
    if cool_above.unwrap() <= heat_below.unwrap() {
        println!("ERROR: Setting Cool Above ({}) to less than or equal to Heat Below ({}).", cool_above.unwrap(), heat_below.unwrap());
        valid = false;
    }
    if interval.unwrap() < 1 {
        println!("ERROR: Setting interval to less than every minute.");
        valid = false;
    }
    if off_above.is_some() && off_below.is_some() {
        if !(cool_above.unwrap() > off_above.unwrap() && off_above.unwrap() > off_below.unwrap() && off_below.unwrap() > heat_below.unwrap()) {
            println!("ERROR: Cool Above > Off Above > Off Below > Heat Below is not true.");
            valid = false;
        }
    }
    else if off_above.is_some() {
        if !(cool_above.unwrap() > off_above.unwrap() && off_above.unwrap() > heat_below.unwrap()) {
            println!("ERROR: Cool Above > Off Above > Heat Below is not true.");
            valid = false;
        }
    }
    else if off_below.is_some() { 
        if !(cool_above.unwrap() > off_below.unwrap() && off_below.unwrap() > heat_below.unwrap()) {
            println!("ERROR: Cool Above > Off Below > Heat Below is not true.");
            valid = false;
        }
    }
    valid
}