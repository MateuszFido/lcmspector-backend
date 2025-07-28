use crate::measurements::Compound;
use mzdata::spectrum::{MultiLayerSpectrum, SpectrumLike};
use mzdata::MzMLReader;
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::time::Instant;
use indicatif::ParallelProgressIterator;

pub fn load_ms_scans(file_path: &str) -> (Vec<MultiLayerSpectrum>, Vec<MultiLayerSpectrum>) {
    let start_time = Instant::now();

    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return (Vec::new(), Vec::new());
        }
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
    
    // Process files in parallel using Rayon
    let results: Vec<(String, Vec<Compound>)> = file_paths
        .par_iter()
        .progress_count(file_paths.len() as u64)
        .map(|file_path| {
            // Load MS scans from the file
            let (ms1_scans, _) = load_ms_scans(file_path);
            
            // Clone the Arc to the ion list to share it across threads
            let ion_list_ref = Arc::clone(&ion_list);
            
            // Process the scans using construct_xics
            let result = crate::processing::construct_xics(&ms1_scans, &ion_list_ref, mass_accuracy);
            
            // Return the file path and result
            (file_path.clone(), result)
        })
        .collect();
    
    println!("Processed {} files in {:.2?} seconds.", file_paths.len(), start_time.elapsed());
    
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
