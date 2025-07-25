use crate::measurements::Compound;
use mzdata::spectrum::{MultiLayerSpectrum, SpectrumLike};
use mzdata::MzMLReader;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

pub fn load_mzml(path: &str, ion_list_name: &str) -> (Vec<MultiLayerSpectrum>, Vec<Compound>) {
    let now = Instant::now();

    // Load MzML file
    let file = File::open(path).expect("Could not open file");
    let reader = MzMLReader::new(file);

    // Parallel processing of scans
    let ms1_scans: Vec<MultiLayerSpectrum> = reader.filter(|scan| scan.ms_level() == 1).collect();

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

    println!(
        "Loaded {} scans and {} compounds in {} seconds.",
        ms1_scans.len(),
        compounds.len(),
        now.elapsed().as_secs_f32()
    );

    (ms1_scans, compounds)
}
