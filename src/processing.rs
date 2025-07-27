use crate::measurements::Compound;
use mzdata::spectrum::MultiLayerSpectrum;
use mzdata::spectrum::SpectrumLike;
use ndarray::Array2;
use std::iter::zip;

pub fn construct_xics<'a>(
    data: &'a [MultiLayerSpectrum],
    ion_list: &'a [Compound],
    mass_accuracy: f64,
) -> Vec<Compound> {
    let mut compounds = ion_list.to_vec();
    for compound in &mut compounds {
        for (ion_name, ion_data) in &mut compound.ions {
            let mut intensities = Vec::new();
            let mut scan_times = Vec::new();
            println!("{ion_name}, {ion_data:?}");
            // Calculate mass range
            let mass = ion_name
                .parse::<f64>()
                .unwrap_or_else(|e| panic!("Failed to parse mass: {e}"));
            let mass_range = (mass - 3.0 * mass_accuracy, mass + 3.0 * mass_accuracy);

            // Safeguard for mass range
            let mass_range = if mass_range.0 < 0.0 {
                (0.0, mass_range.1)
            } else {
                mass_range
            };

            for spectrum in data {
                if let Some(arrays) = spectrum.arrays.as_ref() {
                    for (mz, intensity) in zip(
                        arrays.mzs().unwrap().iter(),
                        arrays.intensities().unwrap().iter(),
                    ) {
                        if *mz >= mass_range.0 && *mz <= mass_range.1 {
                            intensities.push(*intensity as f64);
                            scan_times.push(spectrum.description.acquisition.start_time());
                        }
                    }
                }
            }

            // Create XIC array
            let xic_array =
                Array2::from_shape_vec((2, intensities.len()), [scan_times, intensities].concat())
                    .expect("Failed to create XIC array");

            if xic_array.is_empty() {
                ion_data.insert("MS Intensity".to_string(), None);
                continue;
            }

            // Update compound ion data
            ion_data.insert("MS Intensity".to_string(), Some(xic_array.sum()));

            if ion_data.get("MS Intensity").is_none() || xic_array.shape()[1] < 2 {
                continue;
            }

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
        // open the ion list from ion_lists.json
        let ion_list_file = std::fs::File::open("ion_lists.json")
            .unwrap_or_else(|e| panic!("Couldn't open ion list file: {e}"));
        let ion_list_reader = std::io::BufReader::new(ion_list_file);
        let ion_list: serde_json::Value = serde_json::from_reader(ion_list_reader)
            .unwrap_or_else(|e| panic!("Failed to parse ion list JSON: {e}"));

        // Take short-chain fatty acids
        let ion_list = ion_list["scfas"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(name, compound_data)| Compound {
                name: name.to_string(),
                ions: HashMap::from_iter(
                    compound_data["ions"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|ion| (ion.to_string(), HashMap::new())),
                ),
                ion_info: compound_data["info"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|info| info.as_str().unwrap().to_string())
                    .collect(),
                calibration_curve: HashMap::new(),
                ms2: Vec::new(),
            })
            .collect::<Vec<Compound>>();

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
        assert_eq!(result.len(), 4);
        let compound = &result[0];
        assert_eq!(compound.name, "Acetate");

        let negative_mode = [59.0139, 73.0295, 87.04515];

        // Check that RT and MS Intensity have been populated
        for (ion, ion_data) in compound.ions.iter() {
            if negative_mode
                .iter()
                .any(|negative_mode_ion| ion.parse::<f64>().unwrap() == *negative_mode_ion)
            {
                assert!(ion_data.contains_key("RT"));
                assert!(ion_data.contains_key("MS Intensity"));
                assert!(ion_data.get("MS Intensity").unwrap().is_some());
            }
        }
    }
}
