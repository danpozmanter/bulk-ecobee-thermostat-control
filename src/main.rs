use clap::Parser;
use simplelog::*;
use log::info;


mod ecobee;
mod storage;
mod weather;

/// # Args struct for Clap
/// 
/// Command line arguments.
/// 
/// Stick to the long form for clarity (and to avoid collision).
/// For convenience, allow the short form for `refresh` and `status`.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {

    // Setup & configuration arguments

    #[arg(long, exclusive=true)]
    key: bool,
    
    #[arg(long, exclusive=true)]
    pin: bool,

    #[arg(long, exclusive=true)]
    auth: bool,

    #[arg(long, exclusive=true)]
    weather_setup: bool,

    // Normal runtime arguments

    #[arg(short, long)]
    verbose: bool,

    #[arg(long, conflicts_with_all=["heat", "cool", "off", "refresh", "status"])]
    weather: bool,

    #[arg(short, long)]
    refresh: bool,

    #[arg(short, long)]
    status: bool,

    #[arg(long, conflicts_with_all=["heat", "off", "weather"])]
    cool: bool,

    #[arg(long, conflicts_with_all=["cool", "off", "weather"])]
    heat: bool,

    #[arg(long, conflicts_with_all=["cool", "heat", "weather"])]
    off: bool,
}

fn main() {
    let args = Args::parse();
    
    let log_level = if args.verbose { LevelFilter::Info } else { LevelFilter::Error };
    let log_config = ConfigBuilder::new()
        .set_time_level(LevelFilter::Off)
        .build();

    SimpleLogger::init(log_level, log_config).unwrap();

    info!("Bulk Ecobee Thermostat Control Run @ {}", chrono::offset::Local::now().to_rfc2822());

    // Handle setup first,

    // Setup Step 1
    if args.key {
        ecobee::api::api_key();
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

    // Weather Setup

    if args.weather_setup {
        weather::settings::setup();
        return
    }

    // Weather Mode

    if args.weather {
        weather::api::run();
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
