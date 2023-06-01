use serde::{Deserialize, Serialize};

/// # AuthorizeResponse
/// 
/// ecobeePin   The PIN a user enters in the web portal.
/// expires_in  The number of minutes until the PIN expires. Ensure you inform the user how much time they have.
/// code        The authorization token needed to request the access and refresh tokens.
/// scope       The requested Scope from the original request. This must match the original request.
/// interval    The minimum amount of seconds which must pass between polling attempts for a token. */
#[derive(Deserialize, Debug)]
pub struct AuthorizeResponse {
    #[serde(rename="ecobeePin")]
    pub ecobee_pin: String,  // Matches json
    pub expires_in: u32,
    pub code: String,
    pub scope: String,
    pub interval: u32
}


///  # TokenResponse
/// 
/// {
///     "access_token": "Rc7JE8P7XUgSCPogLOx2VLMfITqQQrjg",
///     "token_type": "Bearer",
///     "expires_in": 3599,
///     "refresh_token": "og2Obost3ucRo1ofo0EDoslGltmFMe2g",
///     "scope": "smartWrite" 
/// }
#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: String,
    pub scope: String
}

/// # Tokens
/// 
/// Convenience struct to pass both access and refresh tokens around.
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String
}

/// # ThermostatMeta
/// 
/// For storing/retrieving basic thermostat metadata (identifier and name).
#[derive(Deserialize, Debug, Serialize)]
pub struct ThermostatMeta {

    pub identifier: String,
    pub name: String
}