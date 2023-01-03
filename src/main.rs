use std::path::PathBuf;
use clap::{Parser, Subcommand};
use log;
use hpdms;

#[derive(Parser)]
#[command(name = "hpdms")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    /// Increase verbosity
    verbose: u8,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug)]
#[derive(Subcommand)]
enum Commands {
    /// Create a new project
    New {
        /// The name of the new project
        name: PathBuf,
        
        /// The path to the folder used to store the data.
        data_path: PathBuf,
        
        /// The path to the folder that will contain the repository.
        #[arg(default_value = ".")]
        repo_path: PathBuf,

        /// The data folder path will not be initialized with a subfolder
        #[arg(short, long)]
        data_path_is_local: bool,
        
        /// Do not include the /report folder with the latex report
        #[arg(short, long)]
        exclude_report: bool,
    },

    /// Package and export an existing project
    Takeout {
        /// The path to the project to export.
        #[arg(default_value = ".")] 
        repo_path: PathBuf,

        /// Also export a compressed file with the original source code
        #[arg(short, long)]
        include_source: bool,
    },

    /// Restore a remote project
    Restore {
        remote_url: String,
        data_file: PathBuf,
        #[arg(short, long)]
        skip_checks: bool
    }
}

#[derive(Debug)]
struct RuntimeError;
fn main() -> Result<(), RuntimeError> {
    let cli = Cli::parse();
    let log_level = match cli.verbose {
        0 => log::Level::Warn,
        1 => log::Level::Info,
        2 => log::Level::Debug,
        _ => log::Level::Trace,
    };

    simple_logger::init_with_level(log_level).unwrap();

    let command = cli.command;
    log::info!("You specified {:?}", command);

    hpdms::hello_world();

    Ok(())
}
