#[macro_use]
extern crate lazy_static;
extern crate reqwest;

use std::env::args;

mod script;

fn arg_parse() -> (String, Vec<String>) {
    let arg_tmp = args().skip(1).fold(
        String::new(),
        |acc, val| acc + " " + &val,
    );
    let mut split = arg_tmp.splitn(2, ": ");
    let command = split.next().expect("You must supply a command");
    let args = split
        .next()
        .map(|val| val.split(" ").map(ToString::to_string).collect())
        .unwrap_or(Vec::new());

    (command[1..].to_string(), args)
}

fn main() {
    let (command, args) = arg_parse();
    script::run_script(command, args);
}
