# Build Instructions

## Prerequisites
- **Build Tool**: Rust toolchain (rustc + cargo) — install via [rustup.rs](https://rustup.rs)
- **Minimum Rust Version**: 1.70+ (2021 edition)
- **System Requirements**: Any OS supported by crossterm (Linux, macOS, Windows)

## Build Steps

### 1. Install Rust (if not installed)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Build Debug
```bash
cd mealprep
cargo build
```

### 3. Build Release
```bash
cargo build --release
```

### 4. Verify Build Success
- **Expected Output**: `Finished` message with no errors
- **Build Artifacts**:
  - Debug: `target/debug/mealprep`
  - Release: `target/release/mealprep`
- **Acceptable Warnings**: Dead code warnings for functions only used by library crate tests (not binary)

## Run
```bash
cargo run          # debug mode
cargo run --release # release mode
./target/release/mealprep  # direct binary
```

## Troubleshooting

### Build Fails with Missing Dependencies
- **Cause**: System libraries for crossterm terminal handling
- **Solution**: On Linux, ensure `libc` dev headers are installed. On macOS, Xcode command line tools.

### Build Fails with Rust Version Error
- **Cause**: Rust toolchain too old
- **Solution**: `rustup update stable`
