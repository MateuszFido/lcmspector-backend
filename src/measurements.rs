use mzdata::spectrum::MultiLayerSpectrum;
use std::collections::HashMap;
use std::fmt;

pub struct MSMeasurement {
    pub mass_accuracy: f32,
    pub data: Vec<MultiLayerSpectrum>,
    pub xics: Vec<Compound>,
}

pub struct LCMeasurement {
    pub lc_scans: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct Compound {
    /// The name of the compound
    name: String,

    /// A dictionary of ions with their associated information
    ions: HashMap<String, HashMap<String, Option<f64>>>,

    /// List of MS2 data (type not specified in original, so using a generic Vec)
    ms2: Vec<()>,

    /// Additional ion information
    ion_info: Vec<()>,

    /// Calibration curve data
    calibration_curve: HashMap<String, f64>,
}

impl Compound {
    /// Constructor method equivalent to __init__
    fn new(name: String, ions: Vec<String>, ion_info: Vec<()>) -> Self {
        Compound {
            name,
            ions: ions
                .into_iter()
                .map(|ion| {
                    (
                        ion,
                        HashMap::from([
                            ("RT".to_string(), None),
                            ("MS Intensity".to_string(), None),
                            ("LC Intensity".to_string(), None),
                        ]),
                    )
                })
                .collect(),
            ms2: Vec::new(),
            ion_info,
            calibration_curve: HashMap::new(),
        }
    }
}

// Optional: Implement Display trait for string representation
impl fmt::Display for Compound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Compound: {}, ions: {:?}, ion info: {:?}",
            self.name, self.ions, self.ion_info
        )
    }
}
