# Rust File Compression with Gzip

This Rust program compresses a file using Gzip compression. Below is an explanation of each part of the code.

## Code Explanation

### External Crate Import

```rust
extern crate flate2;
#This line imports the flate2 crate, which provides functionality for working with compressed data, including Gzip compression.
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader};
use std::time::Instant;
#These are use statements that bring specific items into scope from the imported crates and standard library modules. They are used to avoid writing the full path to these items every time they are used in the code.
fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `Source` `target`");
        return;
    }
    // ...
}
#The main function is the entry point of the program. It first checks if the number of command-line arguments is not equal to 3. If the condition is true, it prints a usage message and exits the program.
let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
let output = File::create(args().nth(2).unwrap()).unwrap();
let mut encoder = GzEncoder::new(output, Compression::default());
let start = Instant::now();
copy(&mut input, &mut encoder).unwrap();
let output = encoder.finish().unwrap();
BufReader::new(File::open(args().nth(1).unwrap()).unwrap()): Opens the source file specified in the first command-line argument for reading and creates a buffered reader.
File::create(args().nth(2).unwrap()).unwrap(): Creates or truncates the target file specified in the second command-line argument for writing.
GzEncoder::new(output, Compression::default()): Creates a Gzip encoder that writes compressed data to the target file.
copy(&mut input, &mut encoder).unwrap(): Copies data from the source file to the Gzip encoder, compressing it in the process.
encoder.finish().unwrap(): Finalizes the compression process and returns the compressed data.
## Output Information
println!("Source len:{:?}", input.get_ref().metadata().unwrap().len());
println!("Target len:{:?}", output.metadata().unwrap().len());
println!("Elapsed time:{:?}", start.elapsed());

```

# Rust File Compression with Gzip

This Rust program compresses a file using Gzip compression. Below is an explanation of each part of the code.

## Code Explanation

### File Handling and Compression

- `BufReader::new(File::open(args().nth(1).unwrap()).unwrap())`: Opens the source file specified in the first command-line argument for reading and creates a buffered reader.
- `File::create(args().nth(2).unwrap()).unwrap()`: Creates or truncates the target file specified in the second command-line argument for writing.
- `GzEncoder::new(output, Compression::default())`: Creates a Gzip encoder that writes compressed data to the target file.
- `copy(&mut input, &mut encoder).unwrap()`: Copies data from the source file to the Gzip encoder, compressing it in the process.
- `encoder.finish().unwrap()`: Finalizes the compression process and returns the compressed data.

---

You can copy this content into a Markdown file for documentation or further reference.
