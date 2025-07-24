use mzdata::prelude::SpectrumLike;
use mzdata::spectrum::MultiLayerSpectrum;
use mzdata::MzMLReader;
use std::fs;
use std::mem::size_of_val;
use std::time::Instant;


pub fn load_mzml() {

    let now = Instant::now();
    let reader = MzMLReader::new(fs::File::open("tests/ANIL_1mM_1_neg.mzML").unwrap());

    // automatically boxed before putting in a vector
    let mut ms1_scans: Vec<MultiLayerSpectrum> = Vec::new(); 

    let mut ms2_scans: Vec<MultiLayerSpectrum> = Vec::new();

    for spectrum in reader {
        if spectrum.ms_level() == 1 {
            ms1_scans.push(spectrum);
        }
        else if spectrum.ms_level() == 2 {
            ms2_scans.push(spectrum);
        }
    }

    println!("Found {} ms1 scans", ms1_scans.len());
    println!("Found {} ms2 scans", ms2_scans.len());
    let size_of_ms1 = size_of_val(&*ms1_scans) as f32 / 1024.0 / 1024.0;
    let size_of_ms2 = size_of_val(&*ms2_scans) as f32 / 1024.0 / 1024.0;
    println!("Size of MS1 scans in bytes: {size_of_ms1} MB");
    println!("Size of MS2 scans in bytes: {size_of_ms2} MB");

    println!("Time elapsed: {} ms", now.elapsed().as_millis());
}