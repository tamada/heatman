# Heatman 🔥

[![build](https://github.com/tamada/heatman/actions/workflows/build.yaml/badge.svg)](https://github.com/tamada/heatman/actions/workflows/build.yaml)
[![Coverage Status](https://coveralls.io/repos/github/tamada/heatman/badge.svg?branch=main)](https://coveralls.io/github/tamada/heatman?branch=main)

[![Version](https://img.shields.io/badge/Version-v0.1.0-green)](https://github.com/tamada/heatman/releases/tag/v0.1.1)
[![License](https://img.shields.io/badge/License-MIT-green)](https://github.com/tamada/heatman/blob/main/LICENSE)

[![Docker](https://img.shields.io/badge/Docker-ghcr.io/tamada/heatman:0.1.1-blue?logo=docker)](https://github.com/tamada/heatman/pkgs/container/heatman/)
[![crates.io](https://img.shields.io/badge/crates.io-heatman-blue?logo=rust)](https://crates.io/crates/heatman)
[![Homebrew](https://img.shields.io/badge/Homebrew-tamada/tap/heatman-blue?logo=homebrew)](https://github.com/tamada/homebrew-tap)

Heatmap generator for visualizing data in a matrix format.

## 📣 Features

- Generate heatmaps from csv files,
- Support for assistant lines to enhance readability,
- Customizable pixel size for cells,
- Specify the order of data to be plotted, and
- Output images in PNG format.

## 🏃 CLI installation & Usage

### ⚓️ Installation

#### 🍺 Homebrew

You can install the CLI tool using Homebrew:

```sh
brew install tamada/tap/heatman
```

### Basic Usage

The heatman CLI tool converts csv files into heatmap images.
The csv file should contain the row and column header and each cell should be a number between 0 and 1.
The output image will be saved as `heatman.png` by default.
The example csv files are available on the `testdata` directory.

```sh
heatman testdata/sample.csv
```

The resultant image is as follows:

![heatman.png](https://github.com/tamada/heatman/raw/main/assets/images/heatman.png)

Other usage examples are available in the [CLI interface documentation](cli/README.md).

### 🐳 Docker available

You can also use `heatman` via Docker without installing it on your system:

```sh
docker run --rm -it -v $PWD:/app ghcr.io/tamada/heatman:latest /app/testdata/sample.csv
```

The details of the Docker image are available in the [Containerfile](Containerfile).

## 👟 Library usage

To use `heatman` in your Rust project, add it to your `Cargo.toml`:

```toml
[dependencies]
heatman = "0.1.0" # Check for the latest version
```

### Example 1: Basic heatmap generation

```rust
use heatman::{Heatmap, Data, DataLoader};
use image::{ImageBuffer, Rgba};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = DataLoader::load("testdata/sample.csv")
        .expect("Failed to load data from CSV");
    let cells: Data<Rgba<u8>> = data.into();
    let context = Heatmap::new(cells, 10);
    let image: ImageBuffer<Rgba<u8>, Vec<u8>> = context.into();
    image.save("output.png")
        .expect("Failed to save heatmap image");
    Ok(())
}
```

This code is available in the `examples/generate_heatmap_from_csv.rs` file.

## ℹ️ About

### 🧑‍💼 Authors 👩‍💼

- [Haruaki Tamada](https://github.com/tamada)

### 📜 License

[![License](https://img.shields.io/badge/License-MIT-green)](https://github.com/tamada/heatman/blob/main/LICENSE)

This project is licensed under the MIT License. See the LICENSE file for details.

### 🎃 Logo

![heatman logo](https://github.com/tamada/heatman/raw/main/assets/images/logo.png)
