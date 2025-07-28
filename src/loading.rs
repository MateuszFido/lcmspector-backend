use crate::measurements::Compound;
use mzdata::spectrum::{MultiLayerSpectrum, SpectrumLike};
use mzdata::MzMLReader;
use rayon::iter::{ParallelIterator, Either, IntoParallelRefMutIterator};
use rayon::prelude::*;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};
use num_cpus;

pub fn load_ms_scans(file_path: &str) -> (Vec<MultiLayerSpectrum>, Vec<MultiLayerSpectrum>) {
    let start_time = Instant::now();

    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => return (Vec::new(), Vec::new()),
    };
    let mzml_reader = MzMLReader::new(file);

    let mut ms1_scans = Vec::new();
    let mut ms2_scans = Vec::new();
    for scan in mzml_reader {
        match scan.ms_level() {
            1 => ms1_scans.push(scan),
            2 => ms2_scans.push(scan),
            _ => (),
        }
    }

    println!(
        "Loaded {} MS1 scans and {} MS2 scans in {:.2?} seconds from {}.",
        ms1_scans.len(),
        ms2_scans.len(),
        start_time.elapsed(),
        file_path
    );

    (ms1_scans, ms2_scans)
}

/// Processes multiple MS files in parallel, returning a vector of results
/// where each result contains the MS1 scans and processed compounds from each file
pub fn process_files_in_parallel(
    file_paths: &[String],
    ion_list_name: &str,
    mass_accuracy: f64,
) -> Vec<(String, Vec<Compound>)> {
    let start_time = Instant::now();
    println!("Starting parallel processing of {} files...", file_paths.len());
    
    // Load ion list once - it will be shared across all file processing tasks
    let ion_list = Arc::new(load_ion_lists(ion_list_name));
    
    let results: Vec<(String, Vec<Compound>)> = file_paths
        .par_iter()
        .map(|file_path| {
            let (ms1_scans, _) = load_ms_scans(file_path);
            let result = crate::processing::construct_xics(
                &ms1_scans, 
                &ion_list, 
                mass_accuracy
            );
            (file_path.clone(), result)
        })
        .collect();

    println!("Parallel processing completed in {:.2?} seconds.", start_time.elapsed());

    results
}

pub fn load_ion_lists(ion_list_name: &str) -> Vec<Compound> {
    // Load ion lists
    let ion_lists_file = File::open("ion_lists.json").expect("Could not open ion lists file");
    let ion_lists_reader = BufReader::new(ion_lists_file);
    let ion_lists: Value =
        serde_json::from_reader(ion_lists_reader).expect("Failed to parse ion lists JSON");

    // Create Compounds from the specified ion list
    let compounds: Vec<Compound> = if let Some(list) = ion_lists.get(ion_list_name) {
        list.as_object()
            .map(|compounds_map| {
                compounds_map
                    .iter()
                    .map(|(name, compound_data)| {
                        let ions = compound_data["ions"]
                            .as_array()
                            .map(|v| v.iter().filter_map(|x| x.as_f64()).collect())
                            .unwrap_or_default();

                        let ion_info = compound_data["info"]
                            .as_array()
                            .map(|v| {
                                v.iter()
                                    .filter_map(|x| x.as_str().map(String::from))
                                    .collect()
                            })
                            .unwrap_or_default();

                        Compound::from_ion_list_entry(name.clone(), &ions, &ion_info)
                    })
                    .collect()
            })
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    compounds
}
