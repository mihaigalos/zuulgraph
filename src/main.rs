extern crate rand;
extern crate yaml_rust;

use rand::Rng;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlLoader;

fn main() {
    let args: Vec<String> = env::args().collect();
    prologue();

    for file in &args[1..] {
        let jobs = load_file(&file);
        generate_dot(jobs);
    }

    epilogue();
}

fn prologue() {
    println!("digraph D {{");
}

fn epilogue() {
    println!("}}");
}

fn load_file(file: &str) -> yaml_rust::Yaml {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0];
    let jobs = doc["jobs"].clone();

    jobs
}

fn get_random_color() -> std::string::String {
    let red = format!("{:X}", rand::thread_rng().gen::<u8>());
    let green = format!("{:X}", rand::thread_rng().gen::<u8>());
    let blue = format!("{:X}", rand::thread_rng().gen::<u8>());

    let hex_color = red + &green + &blue;

    hex_color
}

fn generate_dot(jobs: yaml_rust::Yaml) {
    let mut job_name = "";
    let mut job_parent;

    let hexified_rgb = get_random_color().to_owned();
    let hex_color = "#".to_owned() + &hexified_rgb;

    for job in jobs.as_vec().unwrap() {
        for field in job.as_hash().unwrap() {
            if field.0.as_str() == Some("name") {
                job_name = field.1.as_str().unwrap();
            } else if field.0.as_str() == Some("parent") {
                job_parent = field.1.as_str().unwrap();
                println!("  {} -> {}", job_name, job_parent);
                println!(
                    "  \"{}\" [style=filled, fillcolor=\"{}\"]",
                    job_name, hex_color
                );
            }
        }
    }
}
