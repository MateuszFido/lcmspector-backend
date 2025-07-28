use crate::measurements::Compound;
use mzdata::spectrum::{MultiLayerSpectrum, SpectrumLike};
use mzdata::MzMLReader;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

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
        "Loaded {} MS1 scans and {} MS2 scans in {:.2?} seconds.",
        ms1_scans.len(),
        ms2_scans.len(),
        start_time.elapsed()
    );

    (ms1_scans, ms2_scans)
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
