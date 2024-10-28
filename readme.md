# logo

A command-line tool for overlaying logos on images with precise control over size and position.

## Features

- Add a logo to any image
- Control logo size as a percentage of the base image
- Choose logo position (top-right, top-left, bottom-right, bottom-left)
- Maintains logo aspect ratio
- Supports various image formats (PNG, JPEG, etc.)

## Installation

### Using Cargo (Recommended)

If you have Rust installed, you can install directly from crates.io:

```bash
cargo install ku-logo
```

### Manual Installation

1. Download the latest release for your platform from the [releases page](https://github.com/kualta/logo/releases)
2. Extract the archive
3. Add the binary to your PATH

For development setup, see the [Development Setup](#development-setup) section.

### Development Setup

To set up the development environment, we use [devenv.sh](https://devenv.sh/). This ensures a consistent development experience across different platforms.

1. Install `devenv` following the instructions at [devenv.sh/getting-started](https://devenv.sh/getting-started/)

2. Clone the repository:
   ```bash
   git clone https://github.com/kualta/logo.git
   cd logo
   ```

3. Initialize the development environment:
   ```bash
   devenv init
   ```

4. Enter the development shell:
   ```bash
   devenv shell
   ```

This will set up all required dependencies, including:
- Rust toolchain and cargo
- Cross-compilation tools
- Required system libraries

#### Available Scripts

- `hello` - Test the development environment
- `build-linux` - Build release binary for Linux

Run scripts using:
```bash
devenv run <script-name>
```

## Usage

```bash
# Basic usage with default settings (5% size, top-right position)
logo -i input.png -l logo.png -o output.png

# Specify size and position
logo -i input.png -l logo.png -p 10 --position bottom-left -o output.png

# Show help
logo --help
```

### Options

- `-i, --image <PATH>` - Path to the base image
- `-l, --logo <PATH>` - Path to the logo image
- `-p, --percentage <NUMBER>` - Logo size as percentage (default: 5%)
- `--position <POSITION>` - Logo position (top-right, top-left, bottom-right, bottom-left)
- `-o, --output <PATH>` - Output path for the resulting image

## License

This project is licensed under the MIT License - see the LICENSE file for details.
