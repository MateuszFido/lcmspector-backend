pub mod loading;
pub mod measurements;
pub mod processing;

use crate::loading::{load_ion_lists, load_ms_scans};
use crate::processing::construct_xics;
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
/// - Iterates over each provided MS file path, performing the following:
///   - Loads MS1 and MS2 scans from the file.
///   - Loads the specified ion list.
///   - Constructs extracted ion chromatograms (XICs) using the loaded data.
///   - Prints the resultant compounds to the console.
/// - Exits the program successfully.

fn main() {
    println!(
        "Running LCMSpector backend version {}",
        env!("CARGO_PKG_VERSION")
    );
    let args: Vec<String> = env::args().collect();

    // Require exactly 3 arguments: program name, MS file path, and ion list name
    if args.len() < 3 {
        eprintln!("Usage: {} <ion_list_name>  <ms_file_paths>", args[0]);
        process::exit(1);
    }

    let ion_list_name = &args[1];

    for ms_file_path in &args[2..] {
        println!("Reading from: {ms_file_path}");
        println!("Using ion list: {ion_list_name}");

        let (ms1_scans, _ms2_scans) = load_ms_scans(ms_file_path);
        let compounds = load_ion_lists(ion_list_name);

        let result = construct_xics(&ms1_scans, &compounds, 0.0001);

        for compound in result {
            println!("{}", compound);
        }
    }

    process::exit(0);
}
