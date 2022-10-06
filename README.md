# RS-plotter

Simple one-line plotting tool

![Screenshoot](image/screenshoot.png?raw=true "Screenshoot")

## Installation

### Hard way:
1. Install Rust via [Rustup](https://rustup.rs/)
2. Compile the project with:

```bash
cargo build --release
```

3. Place the executable file in directory: /usr/local/bin (for Linux)
For Windows: place the executable file in any convenient directory, after that add PATH to your Environment variables

### Easy way:
1. Download the executable from the Release section
2. Place the executable file in directory: /usr/local/bin (for Linux)
For Windows: place the executable file in any convenient directory, after that add PATH to your Environment variables

## Usage

```bash
plt <name_of_the_file>
```

The program is allowed to draw plots of files that look like this:
```
# comment
# comment
123     453
# comment
4543    567
946     601
...
```

## License
[MIT](https://choosealicense.com/licenses/mit/)
