use mzdata::spectrum::{MultiLayerSpectrum, SpectrumLike};
use mzdata::MzMLReader;
use std::fs::File;
use std::time::Instant;

pub fn load_mzml(path: &str) -> Vec<MultiLayerSpectrum> {
    let now = Instant::now();

    let file = File::open(path).expect("Could not open file");
    let reader = MzMLReader::new(file);

    // Parallel processing of scans
    let ms1_scans: Vec<MultiLayerSpectrum> = reader.filter(|scan| scan.ms_level() == 1).collect();

    println!(
        "Loaded {} scans in {} seconds.",
        ms1_scans.len(),
        now.elapsed().as_secs_f32()
    );
    ms1_scans
}
