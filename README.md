# RS-plotter

Simple one-line plotting tool

## Installation

### Hard way:
1. Install Rust via [Rustup](https://rustup.rs/)
2. Compile project with:

```bash
cargo build --release
```

3. Place executable file in directory: /usr/local/bin

### Easy way:
1. Download the executable from the Release section
2. Place it in directory: /usr/local/bin

## Usage

```bash
plt <name_of_the_file>
```

The program is allowed to draw plots of files that look like this:
# comment
# comment
123     453
# comment
4543    567
946     601
...

## License
[MIT](https://choosealicense.com/licenses/mit/)