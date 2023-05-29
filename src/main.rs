use clap::Parser;

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
    auth: bool,

    #[arg(long, exclusive=true)]
    authtoken: bool,

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

    // Handle authorization or authtoken first.

    if args.auth {
        ecobee::api::authorize();
        return
    }

    if args.authtoken {
        ecobee::api::get_tokens_with_code();
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
