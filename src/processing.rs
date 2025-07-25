use crate::measurements::Compound;
use mzdata::spectrum::MultiLayerSpectrum;
use ndarray::Array2;

fn construct_xics(
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
