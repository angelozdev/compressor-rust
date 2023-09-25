# Compressor

Welcome to the Compressor! This tool allows you to compress your files using the powerful Rust language. By utilizing this compressor, you can quickly and efficiently transform your files into a `.gz` format, reducing their size and making them easier to share and store.

## Features

- **Fast and Efficient**: Built using Rust, this tool offers top-notch performance.
- **Simple Interface**: Just provide the source file and let the compressor do the rest.
- **Automatic Naming**: The compressed file will have the same name as the original but with a `.gz` extension.

## Installation

Before you run the Compressor, ensure you have Rust and Cargo installed. If not, you can get them [here](https://rustup.rs/).

1. Clone the repository:

```bash
git clone https://github.com/angelozdev/compressor-rust.git
```

2. Navigate to the project directory:

```bash
cd compressor-rust
```

3. Build the project:

```bash
cargo build --release
```

## Usage

To compress a file, use the following command:

```bash
./compressor <source>
```

For example, to compress a file named `file.txt`, you would run:

```bash
./compressor file.txt
```

This will produce a compressed file named `file.txt.gz`.

## Notes

- Ensure the source file exists; otherwise, the compressor will throw an error.
- The compressed file will be located in the same directory as the source file.
- If a file with the name `<source>.gz` already exists, it will be overwritten.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change. Ensure you update the tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
