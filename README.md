# LCMSpector Backend

## Overview

This is a backend of [LCMSpector](https://github.com/MateuszFido/LCMSpector), originally written in Python, which is a high-performance GUI application for processing mass spectrometry data files, specifically designed to handle .mzML file formats. This backend version focuses on extracting and processing MS1 level scans with efficient, parallel processing capabilities and advanced data handling features.

## Features

- Fast processing of .mzML files
- Parallel scan extraction
- MS-level scan filtering
- Performance timing for data loading
- Ion list loading and processing
- JSON serialization support
- Advanced data manipulation with ndarray

## Prerequisites

- Rust (edition 2021 or later)
- Cargo package manager

## Dependencies

- memmap2 (v0.9.7): Memory-mapped file support
- mzdata (v0.56.0): Mass spectrometry data processing
- rayon (v1.10.0): Data parallelism library
- mmap (v0.1.1): Memory mapping utilities
- serde_json (v1.0): JSON serialization
- serde (v1.0): Serialization framework
- ndarray (v0.16.1): N-dimensional array library with serialization support

## Installation

1. Ensure you have Rust and Cargo installed (https://rustup.rs/)
2. Clone the repository:
   ```bash
   git clone https://github.com/MateuszFido/lcmspector-backend.git
   cd lcmspector-backend
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Processing .mzML Files

Run the application by providing the desired ion list (needs to be in ion_lists.json) and a file containing paths to mzML files

```bash
cargo run -- ion_list file_with_paths_to_mzML
```

Example:
```bash
cargo run -- terpenoids file_paths.txt
```

The tool will:
- Load the specified .mzML file(s)
- Extract MS1 level scans
- Print the number of scans and processing time

### Loading Ion Lists

The application supports loading ion lists from JSON files, enabling advanced data processing and analysis:

```rust
let ion_lists = load_ion_lists("path/to/ion_lists.json");
```

## Performance

LCMSpector leverages Rust's parallel processing capabilities to efficiently handle mass spectrometry data files. The `load_mzml` and `load_ion_lists` functions provide precise timing information for each file processed.

## Data Handling

- Supports memory-mapped file reading for efficient large file processing
- Utilizes ndarray for advanced numerical computations
- Provides JSON serialization for easy data interchange

## Testing

A sample test file `ANIL_1mM_1_neg.mzml` is included in the `tests/` directory for demonstration and testing purposes.

## Contributing

Contributions are welcome! Please submit pull requests or open issues on the GitHub repository.

## License

MIT

## Authors

- Mateusz Fido

## Version

0.1.0
