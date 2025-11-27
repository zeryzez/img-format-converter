Simple command-line utility to convert images between formats based on the output file extension.

## How it works
The program reads an input image, infers the desired output format from the extension of the output path, and writes the image in that format.

Main files:
- [src/main.rs](src/main.rs)
- [Cargo.toml](Cargo.toml)


## Installation
Build a release binary:
```bash
cargo build --release
```

## Usage
Run the program with two arguments: input path and output path.
```bash
# using cargo
cargo run -- path/to/input.png path/to/output.jpg

# or the compiled binary
./target/release/universal_convert path/to/input.png path/to/output.jpg
```

The output format is inferred from the extension of `path/to/output.*`. Supported formats depend on the [`image`](https://crates.io/crates/image) crate.

