mod config_management;
mod openai_api;

use clap::{Parser, Subcommand};

// make a get hhtp request using reqwest
// Path: src/main.rs
fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Configure { token } => {
            config_management::create_config_file_in_user_home_dir(token);
        }
        Commands::Ask { task, debug } => {
            println!("task: {}", task);
            let _ = openai_api::make_request(task, debug);
            }
        }
    }

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Configure {
        #[arg(short, long)]
        token: Option<String>,
    },
    Ask {
        #[arg(short, long)]
        task: String,
        #[arg(short, long)]
        debug: Option<bool>,
    },
}
