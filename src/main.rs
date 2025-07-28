pub mod loading;
pub mod measurements;
pub mod processing;

use std::env;
use std::process;

/// Main entry point for the LCMSpector backend application.
///
/// This function initializes the program, parses command line arguments,
/// and processes provided mass spectrometry (.mzML) files using specified ion lists.
/// It performs the following steps:
/// - Prints the current version of the LCMSpector backend.
/// - Collects command line arguments, expecting at least an ion list name and
///   one or more MS file paths.
/// - Validates the number of arguments and exits with an error message if incorrect.
/// - Processes all files in parallel:
///   - Loads MS1 and MS2 scans from each file concurrently.
///   - Uses the specified ion list for all files.
///   - Constructs extracted ion chromatograms (XICs) using the loaded data.
///   - Prints the resultant compounds to the console.
/// - Exits the program successfully.

fn main() {
    println!(
        "Running LCMSpector backend version {}",
        env!("CARGO_PKG_VERSION")
    );
    let args: Vec<String> = env::args().collect();

    // Require 3+ arguments: program name, ion list name, and MS file paths
    if args.len() < 3 {
        eprintln!("Usage: {} <ion_list_name> <ms_file_paths...>", args[0]);
        process::exit(1);
    }

    let ion_list_name = &args[1];
    let file_paths = args[2..].to_vec();
    
    println!("Processing {} files with ion list: {}", file_paths.len(), ion_list_name);
    
    // Process all files in parallel
    let _results = loading::process_files_in_parallel(&file_paths, ion_list_name, 0.0001);
    
    // Print results for each file
    /* 
    for (file_path, compounds) in results {
        println!("\nResults for file: {}", file_path);
        for compound in compounds {
            println!("{}", compound);
        }
    }
    */
    process::exit(0);
}
