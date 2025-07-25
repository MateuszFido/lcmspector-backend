pub mod loading;
pub mod measurements;
pub mod processing;

use crate::loading::load_mzml;
use std::env;

fn main() {
    println!(
        "Running LCMSpector backend version {}",
        env!("CARGO_PKG_VERSION")
    );
    let args: Vec<String> = env::args().collect();

    for (i, file_path) in args.iter().enumerate() {
        if i == 0 {
            continue; // Skip the first argument as it's the program name
        }

        println!("Reading from: {file_path}");
        load_mzml(file_path);
    }
}
