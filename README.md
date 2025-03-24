# HTML Tag Stripper

A simple Rust utility that removes all HTML tags from an HTML file and outputs only the content, with one item per line.

## Description

This program processes HTML files by:
- Removing all HTML tags
- Preserving only the text content
- Outputting each content item on a separate line
- Removing empty lines and extra whitespace

It's useful for extracting plain text from HTML documents for further processing, analysis, or readability.

## Installation

### Prerequisites
- Rust and Cargo (install from [rust-lang.org](https://www.rust-lang.org/tools/install))

### Setup
1. Clone this repository:
   ```
   git clone https://github.com/yourusername/html-tag-stripper.git
   cd html-tag-stripper
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## Usage

Run the program with the path to an HTML file as an argument:

```
cargo run -- path/to/your/file.html
```

Or use the compiled binary directly:

```
./target/release/html-tag-stripper path/to/your/file.html
```

### Example

Input (`example.html`):
```html
<!DOCTYPE html>
<html>
<head>
    <title>Sample Page</title>
</head>
<body>
    <h1>Hello World</h1>
    <p>This is a paragraph.</p>
    <ul>
        <li>Item 1</li>
        <li>Item 2</li>
    </ul>
</body>
</html>
```

Command:
```
cargo run -- example.html
```

Output:
```
Sample Page
Hello World
This is a paragraph.
Item 1
Item 2
```

## How It Works

1. The program reads the specified HTML file
2. It uses regular expressions to identify and remove all HTML tags
3. It replaces tags with newlines to ensure content separation
4. It processes the resulting text to trim whitespace and remove empty lines
5. It outputs each content item to stdout, one per line

## Dependencies

- `regex`: Used for HTML tag pattern matching

## License

This project is licensed under the [GNU General Public License v3.0 (GPL-3.0)](https://www.gnu.org/licenses/gpl-3.0.en.html).

The GPL is a strong copyleft license that requires anyone who distributes your code or a derivative work to make the source available under the same terms. This is particularly suitable for libraries and applications where you want to ensure that all modifications and extensions remain free and open source.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
