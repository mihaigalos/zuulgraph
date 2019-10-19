extern crate yaml_rust;

use std::env;

use std::process;

mod output_writer;
mod random_color_generator;
mod yaml_parser;

mod dot_generator;
use dot_generator::DotGenerator;

mod test_unit;

fn main() {
    let args = get_program_arguments();
    let dot_generator = DotGenerator {};
    println!("{}",dot_generator.run(args));
}

fn get_program_arguments() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: zuulgraph <file1.yaml> <file2.yaml> .. <file3.yaml>");
        process::exit(0x0001);
    }
    args
}
