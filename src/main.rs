pub mod loading;
pub mod measurements;
pub mod processing;

use crate::loading::load_mzml;
use crate::measurements::{LCMeasurement, MSMeasurement};
use crate::processing::construct_xics;

fn main() {
    println!("Running LCMSpector backend version {}", env!("CARGO_PKG_VERSION"));
    
}
