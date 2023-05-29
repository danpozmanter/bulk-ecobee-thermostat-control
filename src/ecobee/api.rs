use serde_json::Value;
use ureq;
use ureq::Error;

use crate::ecobee::models;
use crate::ecobee::storage;

/// # authorize()
/// 
/// https://www.ecobee.com/home/developer/api/documentation/v1/auth/pin-api-authorization.shtml
///
/// Authorize if using for the first time to authorize the app in your console.
pub fn authorize() {
    println!("Authorizing!");
    
    let app_key = storage::load_app_key();

    match ureq::get("https://api.ecobee.com/authorize")
    .query("response_type", "ecobeePin")
    .query("client_id", app_key.as_str())
    .query("scope", "smartWrite").call() {
        Ok(response) => {
            match response.into_json::<models::AuthorizeResponse>() {
                Ok(auth) => {
                    // Write the auth token.
                    storage::write_tokens(auth.code, "".to_string());
                    println!("Ecobee Authorization PIN: {}", auth.ecobee_pin);
                    println!("(expires in {} minutes)", auth.expires_in);
                    println!("Log into the Ecobee web portal and register the application using the PIN in your `My Apps` widget.");
                },
                Err(e) => println!("{e:?}")
            };
        },
        Err(Error::Status(code, response)) => {
            println!("Error with request to authorize: {code}\n{}", response.into_string().unwrap());
        }
        Err(e) => { println!("Transport error: {e}") }
    }
}

/// # fetch_tokens(access_token: String, grant_type: &str)
/// 
/// Call the token API endpoint to retrieve the access and refresh tokens. 
/// 
/// https://www.ecobee.com/home/developer/api/documentation/v1/auth/auth-req-resp.shtml
/// https://www.ecobee.com/home/developer/api/documentation/v1/auth/token-refresh.shtml
/// 
/// API params:
/// 
/// grant_type	This is always "ecobeePin" for this authorization flow.
/// code	    The authorization code obtained from the /authorize request.
/// client_id	This is your unique application key.
/// ecobee_type	(Deprecated)
pub fn fetch_tokens(access_token: String, grant_type: &str) {

    let app_key = storage::load_app_key();
    let token_param = match grant_type {
        "ecobeePin" => "code",
        "refresh_token" => "refresh_token",
        _ => panic!("Bad grant type for fetch_tokens: {grant_type}")
    };

    match ureq::post("https://api.ecobee.com/token")
    .query("grant_type", grant_type)
    .query("client_id", app_key.as_str())
    .query(token_param, access_token.as_str()).call() {
        Ok(response) => {
            match response.into_json::<models::TokenResponse>() {
                Ok(tok) => {
                    storage::write_tokens(tok.access_token, tok.refresh_token);
                    println!("Tokens retrieved successfully. Expires in {} minutes", tok.expires_in);
                },
                Err(e) => println!("{e:?}")
            }
        },
        Err(Error::Status(code, response)) => {
            println!("Error with request for tokens: {code}\n{}", response.into_string().unwrap());
        }
        Err(e) => { println!("Transport error: {e}") }
        }

}

/// # get_tokens_with_code()
/// 
/// Call fetch_token with the code from the authorization call, and a `grant_type` of "ecobeePin" to get the initial access and refresh tokens.
/// 
/// This needs to be called soon after calling authorize().
pub fn get_tokens_with_code() {
    let tokens = storage::load_tokens();

    println!("Requesting access tokens after registering app with PIN");

    fetch_tokens(tokens.access_token, "ecobeePin");
}

/// # refresh_tokens()
/// 
/// Call fetch_token with the refresh token and a `grant_type` of "refresh_token", to refresh the tokens and update the local store.
pub fn refresh_tokens() {
    let tokens = storage::load_tokens();

    println!("Refreshing with '{}'", tokens.refresh_token);

    fetch_tokens(tokens.refresh_token, "refresh_token");
}


/// # thermostat_status()
/// 
/// For every registered thermostat, get the name, identifier, HVAC Mode, Actual Temperature, and Actual Humidity.
/// 
/// https://www.ecobee.com/home/developer/api/documentation/v1/operations/get-thermostats.shtml
pub fn thermostat_status() {
    let tokens = storage::load_tokens();
    let access = format!("Bearer {}", tokens.access_token.as_str());
    match ureq::get("https://api.ecobee.com/1/thermostat")
    .set("Content-Type", "application/json;charset=UTF-8")
    .set("Authorization", access.as_str())    
    .query("json", "{\"selection\":{\"includeAlertsv\":\"true\",\"selectionType\":\"registered\",\"selectionMatch\":\"\",\"includeEvents\":\"true\",\"includeSettings\":\"true\",\"includeRuntime\":\"true\"}}").call() {
        Ok(response) => {
            match response.into_json::<Value>() {
                Ok(resp) => {
                    if let Some(thermostats) = resp["thermostatList"].as_array() {
                        println!("=========================================");
                        println!("Thermostats");
                        println!("=========================================");
                        for thermostat in thermostats {
                            println!("Thermostat {} (id: {})", thermostat["name"], thermostat["identifier"]);
                            println!("HVAC Mode: {}", thermostat["settings"]["hvacMode"]);
                            let temp = thermostat["runtime"]["actualTemperature"].as_f64().unwrap() / 10.0;
                            println!("Actual Temperature: {}, Actual Humidity: {}%\n", temp, thermostat["runtime"]["actualHumidity"]);
                        }
                        println!("=========================================");
                    }
                },
                Err(e) => println!("{e:?}")
            };
        },
        Err(Error::Status(code, response)) => {
            println!("Error with request for thermostats: {}\n{}", code, response.into_string().unwrap());
        }
        Err(e) => { println!("Transport error: {e}") }
    }
}


/// # update_thermostats(mode: &str)
/// 
/// For every registered thermostat, set the HVAC Mode to the provided string `mode`.
/// 
/// https://www.ecobee.com/home/developer/api/documentation/v1/operations/post-update-thermostats.shtml
pub fn update_thermostats(mode: &str) {
    let tokens = storage::load_tokens();
    let access = format!("Bearer {}", tokens.access_token.as_str());
    match ureq::post("https://api.ecobee.com/1/thermostat")
    .set("Content-Type", "application/json;charset=UTF-8")
    .set("Authorization", access.as_str())    
    .query("format", "json").send_json(ureq::json!({
        "selection": {
            "selectionType": "registered",
            "selectionMatch": "",
        },
        "thermostat": {
            "settings": {
                "hvacMode": mode
            }
        }
    })) {
        Ok(response) => {
            match response.into_string() {
                Ok(resp) => {
                    println!("{resp}");
                },
                Err(e) => println!("{e:?}")
            };
        },
        Err(Error::Status(code, response)) => {
            println!("Error with request for thermostats: {}\n{}", code, response.into_string().unwrap());
        }
        Err(e) => { println!("Transport error: {e}") }
    }

}