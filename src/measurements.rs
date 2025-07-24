use mzdata::spectrum::MultiLayerSpectrum;

pub struct MSMeasurement {
    pub ms1_scans: Vec<MultiLayerSpectrum>,
    pub ms2_scans: Vec<MultiLayerSpectrum>
}

pub struct LCMeasurement {
    pub lc_scans: Vec<u32>
}