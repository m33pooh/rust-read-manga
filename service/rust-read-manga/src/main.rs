// Declare the modules we created
mod cli;
mod converter;
mod error;

// Import the argument parser
use clap::Parser;
use cli::CliArgs;

fn main() {
    // 1. Parse command-line arguments
    let args = CliArgs::parse();

    // You can print the parsed args to test:
    // println!("Input directory: {:?}", args.input_dir);
    // println!("Output file: {:?}", args.output_file);
    // println!("Duration per image: {}s", args.duration);

    // 2. Call the main conversion logic
    // We'll pass the parsed 'args' to our converter
    println!("Starting conversion...");
    
    // Use 'if let Err' to catch and print any errors from the converter
    if let Err(e) = converter::run_conversion(args) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }

    println!("Conversion finished successfully!");
}