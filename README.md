# LCMSpector Backend

## Overview

LCMSpector is a high-performance Rust-based backend tool for processing mass spectrometry data files, specifically designed to handle .mzML file formats. The tool focuses on extracting and processing MS1 level scans with efficient, parallel processing capabilities.

## Features

- Fast processing of .mzML files
- Parallel scan extraction
- MS1 level scan filtering
- Performance timing for data loading

## Prerequisites

- Rust (edition 2021 or later)
- Cargo package manager

## Dependencies

- memmap2 (v0.9.7): Memory-mapped file support
- mzdata (v0.56.0): Mass spectrometry data processing
- rayon (v1.10.0): Data parallelism library

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

Run the application by providing one or more .mzML files as arguments:

```bash
cargo run -- path/to/file1.mzml path/to/file2.mzml
```

Example:
```bash
cargo run -- tests/ANIL_1mM_1_neg.mzml
```

The tool will:
- Load the specified .mzML file(s)
- Extract MS1 level scans
- Print the number of scans and processing time

## Performance

LCMSpector uses Rust's parallel processing capabilities to efficiently handle mass spectrometry data files. The `load_mzml` function provides timing information for each file processed.

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
