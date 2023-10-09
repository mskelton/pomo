mod cmd;
mod config;
mod notifications;
mod status;
mod time;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Custom path for the config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Disable emoji in the output
    #[arg(long)]
    no_emoji: bool,

    /// Enable push notifications
    #[arg(short, long)]
    notify: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Starts a pomodoro break
    Break {
        /// Enable push notifications
        #[arg(short, long)]
        notify: bool,

        /// Custom duration for the break
        #[arg(index = 1)]
        duration: Option<String>,
    },
    /// Changes the duration of the current timer
    Duration {
        /// Custom duration for the focus session
        #[arg(index = 1)]
        duration: Option<String>,
    },
    /// Starts a pomodoro session
    Start {
        /// Enable push notifications
        #[arg(short, long)]
        notify: bool,

        /// Custom duration for the focus session
        #[arg(index = 1)]
        duration: Option<String>,
    },
    /// Toggles to either a new focus session or break
    Toggle {
        /// Enable push notifications
        #[arg(short, long)]
        notify: bool,

        /// Custom duration for the focus session or break
        #[arg(index = 1)]
        duration: Option<String>,
    },
    /// Stops the current pomodoro session
    Stop {
        /// Enable push notifications
        #[arg(short, long)]
        notify: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Break { duration, notify }) => {
            cmd::start_break(duration, notify);
        }
        Some(Commands::Duration { duration }) => {
            cmd::change_duration(duration);
        }
        Some(Commands::Start { duration, notify }) => {
            cmd::start_focus(duration, notify)
        }
        Some(Commands::Toggle { duration, notify }) => {
            cmd::toggle_session(duration, notify)
        }
        Some(Commands::Stop { notify }) => {
            cmd::stop_session(notify);
        }
        None => {
            cmd::print_status(cli.no_emoji, cli.notify);
        }
    }
}
