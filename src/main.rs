mod cmd;
mod config;
mod notifications;
mod status;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Custom path for the config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Disable emoji in the output
    #[arg(long)]
    no_emoji: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Starts a pomodoro break
    Break {
        /// Display a push notification
        #[arg(short, long)]
        notify: bool,

        /// Custom duration for the break
        #[arg(index = 1)]
        duration: Option<String>,
    },
    /// Changes the duration of the current timer
    Duration {},
    /// Starts a pomodoro session
    Start {
        /// Display a push notification
        #[arg(short, long)]
        notify: bool,
    },
    /// Stops the current pomodoro session
    Stop {
        /// Display a push notification
        #[arg(short, long)]
        notify: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Break { duration, notify }) => {
            cmd::start_break(duration, *notify);
        }
        Some(Commands::Duration {}) => cmd::change_duration(),
        Some(Commands::Start { notify }) => cmd::start_session(*notify),
        Some(Commands::Stop { notify }) => cmd::stop_session(*notify),
        None => cmd::print_status(cli.no_emoji),
    }
}
