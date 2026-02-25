use clap::Parser;
use demoji::cli::args::Args;
use demoji::config::Config;
use demoji::core::error::DemojiError;
use demoji::run;

fn main() {
    // Parse command-line arguments
    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(2);
        }
    };

    // Load configuration with error handling
    let config = match Config::load() {
        Ok(config) => config,
        Err(e) => {
            // Check if it's a DemojiError for better messaging
            if let Some(demoji_err) = e.downcast_ref::<DemojiError>() {
                eprintln!("{}", demoji_err.user_message());
            } else {
                eprintln!("Error: Failed to load configuration: {}", e);
            }
            std::process::exit(2);
        }
    };

    // Run the application and get exit code
    match run(args, config) {
        Ok(exit_code) => {
            std::process::exit(exit_code);
        }
        Err(e) => {
            // Check if it's a DemojiError for better messaging
            if let Some(demoji_err) = e.downcast_ref::<DemojiError>() {
                eprintln!("{}", demoji_err.user_message());
            } else {
                eprintln!("Error: {}", e);
            }
            std::process::exit(2);
        }
    }
}
