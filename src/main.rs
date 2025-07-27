pub mod loading;
pub mod measurements;
pub mod processing;

use crate::loading::{load_ion_lists, load_ms1_scans};
use crate::processing::construct_xics;
use std::env;
use std::process;

fn main() {
    println!(
        "Running LCMSpector backend version {}",
        env!("CARGO_PKG_VERSION")
    );
    let args: Vec<String> = env::args().collect();

    // Require exactly 3 arguments: program name, MS file path, and ion list name
    if args.len() != 3 {
        eprintln!("Usage: {} <ms_file_path> <ion_list_name>", args[0]);
        process::exit(1);
    }

    let ms_file_path = &args[1];
    let ion_list_name = &args[2];

    println!("Reading from: {ms_file_path}");
    println!("Using ion list: {ion_list_name}");

    let ms1_scans = load_ms1_scans(ms_file_path);
    let compounds = load_ion_lists(ion_list_name);

    let result = construct_xics(&ms1_scans, &compounds, 0.0001);

    println!("Done");
    for compound in &result {
        println!("{compound}");
    }

    process::exit(0);
}
