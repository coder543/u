use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn output_file() -> File {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("scripts.rs");
    File::create(&dest_path).unwrap()
}

fn input_file() -> String {
    let mut buffer = String::new();
    File::open("src/scripts.rs")
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();
    buffer
}

fn main() {
    println!("cargo:rerun-if-changed=src/scripts.rs");
    let mut output = output_file();
    let input = input_file();
    write!(output, "{{").unwrap();
    let mut buffer = String::new();
    for line in input.lines() {
        if line.starts_with("use") {
            writeln!(output, "{}", line).unwrap();
            continue;
        }
        if line.starts_with("\"") {
            buffer += "script!(";
        }
        if line.starts_with("}") {
            buffer += "});";
            writeln!(output, "{}", buffer).unwrap();
            buffer.clear();
            continue;
        }
        buffer = buffer + line + "\n";
    }
    writeln!(output, "}}").unwrap();
}
