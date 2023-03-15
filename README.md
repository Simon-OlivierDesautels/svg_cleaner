[![Issues][issues-shield]][issues-url]
[![LinkedIn][linkedin-shield]][linkedin-url]

## About The Project

This is a command-line tool for modifying SVG files in bulk. The tool reads all SVG files in the `svg` folder, applies various modifications to the files, and saves the modified files to the `svg_output` folder.

<br>

## Goal

The aim of this command-line tool is to remove any clipPath elements and generic classes from SVG files that can cause conflicts when multiple SVGs are used on the same web page, ensuring seamless integration and preventing issues.

<br>

## Personal Goal

I'm trying to understand python code from a coworker.

<br>

## Requirements

To run this tool, you'll need to have Rust installed on your system. You can download Rust from the official website or via a package manager.

<br>

## Installation

1. Clone this repository to your local machine using the following command:

```
git clone https://github.com/Simon-OlivierDesautels/svg_cleaner
```

2. Navigate to the root directory of the project using the terminal.

3. Run the following command to build the project:

```
cargo build --release
```

4. Once the project is built, you can run the tool using the following command:

```
./target/release/svg-modifier
```

This command will modify all SVG files in the `svg` folder and save the modified files to the `svg_output` folder.

<br>

## Modifications

The tool applies the following modifications to each SVG file:

- Adds a random 6-character string to the end of all class names that start with `.st`.
- Removes all `clipPath` elements from the SVG file.
- Removes all `clip-path` attributes from the SVG file.

<br>

## License

This project is licensed under the MIT License. See the LICENSE.md file for details.

<br>

## Contact

Simon-Olivier Desautels - simonolivier.desautels@gmail.com

Project Link: [https://github.com/Simon-OlivierDesautels/svg_cleaner](https://github.com/Simon-OlivierDesautels/svg_cleaner)

<p align="right">(<a href="#readme-top">back to top</a>)</p>
