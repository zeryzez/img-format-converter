Rust utility to convert images between formats. The project provides two modes:

- CLI mode: convert a single file by passing input and output paths.
- Web server mode: run an HTTP server with a simple web UI to convert files in the browser (batch support).

## How it works

The program reads image data, determines the desired output format (from the CLI output path extension or from a format field in the web form), and writes the image in the chosen format using the `image` crate.

Main source files:

- `src/main.rs` — program entry point (dispatches to CLI or server mode)
- `src/cli.rs` — CLI conversion logic
- `src/server.rs` — web server and upload/convert handlers
- `index.html` — web UI for batch conversions

## Requirements

- Rust (stable toolchain)
- Network access only required for the web UI when running locally in a browser

## Installation (build)

Build a release binary:

```bash
cargo build --release
```

The compiled binary will be available at `target/release/universal_convert`.

## Usage

CLI mode — convert a single file:

```bash
# with cargo
cargo run -- path/to/input.png path/to/output.jpg

# or with the compiled binary
./target/release/universal_convert path/to/input.png path/to/output.jpg
```

Server mode — start the web UI for batch conversions:

```bash
# with cargo
cargo run -- server

# or with the compiled binary
./target/release/universal_convert server
```

Then open `http://127.0.0.1:8080` in your browser. You can drag-and-drop files, choose target formats, convert, and download individual results or a ZIP of all converted files.

## Supported output formats

Supported formats depend on the `image` crate. Common formats included are: `png`, `jpg`/`jpeg`, `gif`, `bmp`, `ico`.
