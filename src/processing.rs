use crate::measurements::Compound;
use mzdata::spectrum::{MultiLayerSpectrum, SpectrumLike};
use ndarray::Array2;

pub fn construct_xics(
    data: &Vec<MultiLayerSpectrum>,
    ion_list: &Vec<Compound>,
    mass_accuracy: f64,
) -> Vec<Compound> {
    let mut compounds = ion_list.clone();

    for compound in &mut compounds {
        for (ion, ion_data) in compound.ions.iter_mut() {
            let mut xic: Vec<f64> = Vec::new();
            let mut scan_id = Vec::new();

            // Calculate mass range
            let mass = ion.parse::<f64>().unwrap();
            let mass_range = (mass - 3.0 * mass_accuracy, mass + 3.0 * mass_accuracy);

            // Safeguard for mass range
            let mass_range = if mass_range.0 < 0.0 {
                (0.0, mass_range.1)
            } else {
                mass_range
            };

            for scan in data.iter() {
                for peakset in scan.peaks.iter() {
                    for peak in peakset.peaks.iter() {
                        if peak.mz >= mass_range.0 && peak.mz <= mass_range.1 {
                            xic.push(peak.intensity.into());
                            scan_id.push(scan.description.acquisition.start_time());
                        }
                    }
                }
            }

            // Create XIC array
            let xic_array = Array2::from_shape_vec((2, xic.len()), [scan_id, xic].concat())
                .expect("Failed to create XIC array");

            if xic_array.is_empty() {
                ion_data.insert("MS Intensity".to_string(), None);
                continue;
            }

            // Update compound ion data
            ion_data.insert("MS Intensity".to_string(), Some(xic_array.sum()));

            // Find max intensity index and get corresponding scan time
            let max_index = xic_array
                .column(1)
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(index, _)| index);

            let rt = if let Some(index) = max_index {
                data[index].description.acquisition.start_time()
            } else {
                0.0
            };

            ion_data.insert("RT".to_string(), Some(rt));
        }
    }
    compounds
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_construct_xics() {
        // Create mock Compound
        let mut ions = HashMap::new();
        ions.insert(
            "49.5028".to_string(),
            HashMap::from([("Mass".to_string(), Some(49.5028))]),
        );
        let ion_list = vec![Compound {
            name: "Test Compound".to_string(),
            ions,
            ms2: Vec::new(),
            ion_info: Vec::new(),
            calibration_curve: HashMap::new(),
        }];

        // Read data in from tests/ANIL_1mM_1_neg.mzml
        let file = std::fs::File::open("tests/ANIL_1mM_1_neg.mzml").unwrap();
        let mut reader = mzdata::MzMLReader::new(file);
        let mut data = Vec::new();
        for scan in &mut reader {
            if scan.ms_level() == 1 {
                data.push(scan);
            }
        }

        // Call construct_xics
        let result = construct_xics(&data, &ion_list, 0.0001);

        // Verify results
        assert_eq!(result.len(), 1);
        let compound = &result[0];
        assert_eq!(compound.name, "Test Compound");

        // Check that RT and MS Intensity have been populated
        for (_, ion_data) in &compound.ions {
            assert!(ion_data.contains_key("RT"));
            assert!(ion_data.contains_key("MS Intensity"));
        }
    }
}
