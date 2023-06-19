use clap::Parser;
use chrono;
use std::io;

mod ecobee;

/// # Args struct for Clap
/// 
/// Command line arguments.
/// 
/// Stick to the long form for clarity (and to avoid collision).
/// For convenience, allow the short form for `refresh` and `status`.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, exclusive=true)]
    apikey: bool,
    
    #[arg(long, exclusive=true)]
    pin: bool,

    #[arg(long, exclusive=true)]
    auth: bool,

    #[arg(short, long)]
    refresh: bool,

    #[arg(short, long)]
    status: bool,

    #[arg(long, conflicts_with_all=["heat", "off"])]
    cool: bool,

    #[arg(long, conflicts_with_all=["cool", "off"])]
    heat: bool,

    #[arg(long, conflicts_with_all=["cool", "heat"])]
    off: bool,
}

fn main() {
    let args = Args::parse();

    println!("Bulk Ecobee Thermostat Control Run @ {}", chrono::offset::Local::now().to_rfc2822());

    // Handle setup first,

    // Setup Step 1
    if args.apikey {
        // Get the API Key from the user, and store it locally for future use.
        // Create the configuration directory if it doesn't yet exist (silently).
        println!("Enter your API Key from the Developer section of the Ecobee consumer portal: ");
        let mut api_key = String::new();
        io::stdin().read_line(&mut api_key).unwrap();
        api_key.truncate(api_key.len() - 1);
        println!("You entered {api_key}\nProceed? (y/n)");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();
        answer.truncate(answer.len() - 1);
        if answer == "y" {
            ecobee::storage::create_config_dir();
            ecobee::storage::write_api_key(api_key);
        }
        return
    }

    // Setup Step 2
    if args.pin {
        ecobee::api::authorize();
        return
    }

    // Setup Step 3
    if args.auth {
        ecobee::api::get_tokens_with_code();
        ecobee::api::thermostat_status(); // Get status, and refresh thermostat data locally.
        return
    }

    // Run refresh and status if they were specified.

    if args.refresh {
        ecobee::api::refresh_tokens();
    }

    if args.status {
        ecobee::api::thermostat_status();
    }

    // If an argument to change the hvac mode is present, apply it and exit.

    if args.cool {
        ecobee::api::update_thermostats("cool");
        return
    }

    if args.heat {        
        ecobee::api::update_thermostats("heat");
        return
    }

    if args.off {
        ecobee::api::update_thermostats("off");
    }
}
