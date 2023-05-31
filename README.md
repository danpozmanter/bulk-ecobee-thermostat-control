## Bulk Ecobee Thermostat Control

This is a utility meant to make it easy to change the hvac mode of Ecobee thermostats all at once.

There's an [API](https://www.ecobee.com/home/developer/api/introduction/index.shtml).

In combination with a scheduling tool like `at`, `cron`, or similar, it's easy to set a custom schedule.

### Stack

* [Rust](https://www.rust-lang.org/)

* [Clap](https://github.com/clap-rs/clap) - Command line argument parsing.

* [Serde](https://github.com/serde-rs/serde) - JSON.

* [UReq](https://github.com/algesten/ureq) - HTTP Requests.

* [Home](https://crates.io/crates/home) - Cross platform friendly home directory.

### Setup and Installation

Clone this repo, and run `cargo build --release`.
[Install Rust](https://rustup.rs/)
Move the resulting binary to the directory of your choice (eg "~/bin"),
and optionally alias or rename (eg to `thermoctl`).

### Ecobee Developer and API/APP Key

Become an [Ecobee Developer](https://www.ecobee.com/en-us/developers/).

Log into your [Consumer Portal](https://www.ecobee.com/consumerportal/) and navigate the menu to "Developer" and create a new app with an associated key.
Copy this key into your HOME directory, in a file named ".ecobee_api_key".

### Authorization and Access Tokens

The initial access code, and then the access and refresh tokens, are stored in ".ecobee_api_tokens" in your HOME directory.

#### Step 1

Request an authorization PIN by running the `--auth` command.
Once received, you will have until "expires_in" is up to enter it into the Ecobee portal to register the app.
This step also authorizes the app (and may be needed if the request and refresh tokens expire).

```bash
thermoctl --auth
```

#### Step 2

Before the temporary access code expires (usually short, as in 9 minutes), run the following to get
an access token and a refresh token.

```bash
thermoctl --authtoken
```

#### Refresh

Refresh tokens no longer expire for the Ecobee API (v1).
This command can be run in tandem with cool, heat, or off.
It will always execute first.

```bash
thermoctl --refresh
thermoctl -r
```

#### Status

Can be used with other commands. Will always execute after refresh, and before a hvac mode change.
(Note: These changes often take a while to take effect - in the mobile app there's often a delay longer than the app expects,
as I will get a failure notice for an hvac mode change only to see it take effect a second later).

Displays the name, identifier, hvac mode, and current temperature and humidity for each thermostat.

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

### Scheduling

This tool pairs well with `cron`, `at`, or another scheduling tool.

For example:

```bash
echo "thermoctl --heat >> ~/logs/ecobee.log" | at midnight
echo "thermoctl -r --cool >> ~/logs/ecobee.log" | at 10am
```

### General Notes

Be mindful not to spam the endpoints. The "--status" command uses the /thermostat endpoint, which explicitly asks not to be used for polling.

### Limitations

* Access tokens typically say "3600" for expiration, but I haven't seen it last beyond an hour.
Which means either that field measures in different units for different endpoints, or it is unreliable.
For this reason, I have built the refresh step as a manual one.
(Typically this happens under the hood).

* The status command doesn't handle pagination. I'm not sure how practical a need there is here. The Go client doesn't implement this either.
The API allows for it however.

* The API will sometimes report success, but one or more thermostats may not actually change over. The mobile app will report the thermostat in the desired state,
but the thermostat itself will report it's previous state. This could be a bug, or it could be a result of a network error if one or more of the thermostats temporarily lose their wifi connection.

### Further Exploration

For a more general purpose implementation and reference, here is [Go-Ecobee](https://github.com/rspier/go-ecobee)
