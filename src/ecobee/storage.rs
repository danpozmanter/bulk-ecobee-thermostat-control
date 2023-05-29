use home;
use std::fs::File;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;
use std::str::FromStr;

use super::models::Tokens;

static API_FILENAME: &str = ".ecobee_api_key";
static TOKENS_FILENAME: &str = ".ecobee_api_tokens";


/// # get_home_file_path(filename: &str) -> String
/// 
/// Get the absolute file path for the filename in the home directory
fn get_home_file_path(filename: &str) -> String {
    match home::home_dir() {
        Some(path) => { 
            match path.as_path().to_str() {
                Some(home_path) => format!("{}/{filename}", String::from_str(home_path).unwrap()),
                None => panic!("Error getting your home directory.")
            }
        },
        None => panic!("Error getting your home directory."),
    }
}


/// # load_app_key() -> String
/// 
/// Load the app/api key from local storage.
pub fn load_app_key() -> String {
    let mut file = match File::open(get_home_file_path(API_FILENAME)) {
        Ok(f) => f,
        Err(e) => { panic!("Error opening api key file: {e}"); }
    };
    let mut contents = String::new();
    
    match file.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(e) => { panic!("Error reading ecobee_api_key file contents: {e}"); }
    }
}

/// # load_tokens() -> Tokens
/// 
/// Load access and refresh tokens (or just the initial access code and a blank string) from local storage.
pub fn load_tokens() -> Tokens {
    let mut file = match File::open(get_home_file_path(TOKENS_FILENAME)) {
        Ok(f) => f,
        Err(e) => { panic!("Error opening api key file: {e}"); }
    };
    let mut contents = String::new();
    
    let content = match file.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(e) => { panic!("Error reading api key file contents: {e}"); }
    };

    let mut csplit = content.split('\n');
    let access_token = csplit.next();
    let refresh_token = csplit.next();

    Tokens { access_token: access_token.unwrap().to_string(), refresh_token: refresh_token.unwrap().to_string()}
}

/// # write_tokens(access_token: String, refresh_token: String)
/// 
/// Write the updated access and refresh tokens to local storage.
pub fn write_tokens(access_token: String, refresh_token: String) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(get_home_file_path(TOKENS_FILENAME));
    match file {
        Ok(mut f) => {
            match write!(f, "{access_token}\n{refresh_token}") {
                Ok(_) => println!("Successfully wrote access tokens:\nAccess: {access_token}\nRefresh: {refresh_token}"),
                Err(e) => panic!("Error writing access tokens {:?}", e.to_string())
            }
        },
        Err(e) => panic!("Error writing access tokens {:?}", e.to_string())
    }
}