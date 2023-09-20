## Bulk Ecobee Thermostat Control

This is a utility meant to make it easy to change the hvac mode of Ecobee thermostats all at once.

There's an [API](https://www.ecobee.com/home/developer/api/introduction/index.shtml).

In combination with a scheduling tool like `at`, `cron`, or similar, it's easy to set a custom schedule.

Alternatively you can now use **weather mode** after setting up a [weatherapi.com](https://www.weatherapi.com/) account.
This mode will automatically change your hvac mode (cool, heat, off) based on the local weather and preset temperature thresholds.

### Stack

* [Rust](https://www.rust-lang.org/)

* [Clap](https://github.com/clap-rs/clap) - Command line argument parsing.

* [Serde](https://github.com/serde-rs/serde) - JSON.

* [UReq](https://github.com/algesten/ureq) - HTTP Requests.

* [Home](https://crates.io/crates/home) - Cross platform friendly home directory.

* [Chrono](https://github.com/chronotope/chrono) - Date and time library for Rust.

* [Log](https://docs.rs/log/latest/log/) & [SimpleLog](https://github.com/drakulix/simplelog.rs) - Logging.

### Setup and Installation

Clone this repo, and run `cargo build --release`.
[Install Rust](https://rustup.rs/)
Move the resulting binary to the directory of your choice (eg "~/bin"),
and optionally alias or rename (eg to `thermoctl`).

### Ecobee Developer and API Key


### Setup - Authorization and Access Tokens

The initial access code, and then the access and refresh tokens, are stored in ".ecobee_bulk_thermostat_control/api_tokens" in your HOME directory.

#### Step 1

Become an [Ecobee Developer](https://www.ecobee.com/en-us/developers/).

Log into your [Consumer Portal](https://www.ecobee.com/consumerportal/) and navigate the menu to "Developer" and create a new app with an associated key.

Run

```bash
thermoctl --key
```

And enter your API key from your newly created app.

#### Step 2

Request an authorization PIN by running the `--pin` command.
Once received, you will have until "expires_in" is up to enter it into the Ecobee portal to register the app.
This step also authorizes the app (and may be needed if the request and refresh tokens expire).

```bash
thermoctl --pin
```

#### Step 3

Before the temporary access code expires (usually short, as in 9 minutes), run the following to get
an access token and a refresh token, as well as complete set up.

```bash
thermoctl --auth
```

#### Verbose || Debug

Can be used with other commands.
If debug is specified, the log level will include debugging information. (`Debug`).
Otherwise, if verbose is specified, info logs will be included. (`Info`).
The default (neither specified) will set logging to `Error`.

```bash
thermoctl --verbose
thermoctl -v
thermoctl --debug
thermoctl -d
```

#### Refresh

Manually refresh tokens no longer expire for the Ecobee API (v1).
This command will be run automatically for weather mode, and for heat/cool/off.

```bash
thermoctl --refresh
thermoctl -r
```

#### Status

Can be used with other commands. Will always execute after refresh, and before a hvac mode change.
(Note: These changes often take a while to take effect - in the mobile app there's often a delay longer than the app expects,
as I will get a failure notice for an hvac mode change only to see it take effect a second later).

Displays the name, identifier, hvac mode, and current temperature and humidity for each thermostat.

Status also refreshes the local store of thermostat names and identifiers. If you add or remove a thermostat, call status before calling update.

```bash
thermoctl --status
thermoctl -s
```

### Set the HVAC Mode

Set your thermostats to cooling, heating, or off manually:

```bash
thermoctl --cool
thermoctl --heat
thermoctl --off
```

### Manual Scheduling

This tool pairs well with `cron`, `at`, or another scheduling tool.

For example:

```bash
echo "thermoctl --heat >> ~/logs/ecobee.log" | at midnight
echo "thermoctl -r --cool >> ~/logs/ecobee.log" | at 10am
```

### Weather Mode

Run the tool in the background, checking the weather and changing the hvac mode based on thresholds.

**WARNING**: Be careful with weather mode not to set the interval too high, or the threshold too close together (or overlapping), or you could run into trouble.

To use weather mode, you will need to run:

```bash
thermoctl --weather-setup
```

You will be prompted to enter your API key for [Weather API](https://www.weatherapi.com/) (or if you have one already, keep it the same).
You will then be prompted to enter your zipcode, desired heat temp, and desired cool temp, and weather API check interval.

You might enter:

```
Leave blank to leave a field unchanged.
weatherapi api key (current value: dkOERJKFjiejf)>
query (current value: unset)> 11102
metric (current value: false)>
heat below (current value: unset)> 56
cool above (current value: unset)> 70
turn hvac off below (current value: unset)> 
turn hvac off above (current value: unset)> 
interval (in minutes) (current value: unset)> 30
```

(This mode automatically refreshes your tokens before each call).

```bash
themoctl --weather
```

This will check every 30 minutes using weather for zipcode 11102. If the outside temperature below 56 F, switch the mode to heating (if it isn't already), and switch the mode to cooling (if it isn't already) when the outside temperature goes above 70 F.

**Query Note**

The Weather API supports any of the following for query:

```
Latitude and Longitude (Decimal degree) e.g: 48.8567,2.3508
city name e.g.: Paris
US zip e.g.: 10001
UK postcode e.g: SW1
Canada postal code e.g: G2J
metar:<metar code> e.g: metar:EGLL
iata:<3 digit airport code> e.g: iata:DXB
auto:ip IP lookup e.g: auto:ip
IP address (IPv4 and IPv6 supported) e.g: 100.0.0.1
```

```bash
thermoctl --check-weather
```

Hits the weather API, and outputs the results.

### General Notes

Be mindful not to spam the endpoints. The "--status" command uses the /thermostat endpoint, which explicitly asks not to be used for polling.

### Limitations

* Access tokens typically say "3600" for expiration, but I haven't seen it last beyond an hour.
Which means either that field measures in different units for different endpoints, or it is unreliable.
For this reason, I have built the refresh step as a manual one.

* The status command doesn't handle pagination. I'm not sure how practical a need there is here. The Go client doesn't implement this either.
The API allows for it however.

* The API allows for bulk updates, but in practice some thermostats may not change (and may falsley report their status to this app and the mobile app).
To get around this, thermostat identifiers are used to make individual calls to each thermostat. This successfully bypasses the issue,
at the cost of additional API calls. (A previous version made a single bulk call - which inevitably left some thermostats changed, and some unchanged but reporting they had
changed in the app. The thermostat itself had the correct setting displayed).

* I have yet to implement logging or control over command output verbosity.

### Further Exploration

For a more general purpose implementation and reference, here is [Go-Ecobee](https://github.com/rspier/go-ecobee)
