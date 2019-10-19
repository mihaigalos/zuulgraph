extern crate yaml_rust;

use std::fs::File;
use std::io::prelude::*;
use yaml_rust::{YamlLoader};
use yaml_rust::yaml::{Hash, Yaml};

fn main() {
    
    prologue();
    load_file("./test/yaml/demo_file1.yaml");
    load_file("./test/yaml/demo_file2.yaml");
    epilogue();
}

fn prologue() {
    println!("digraph D {{");
}

fn epilogue() {
    println!("}}");
}

fn load_file(file: &str) {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0];
    let jobs= &doc["jobs"];
    
    let mut job_name = "";
    let mut job_parent;

    for job in jobs.as_vec().unwrap() {
        for field in job.as_hash().unwrap()
        {
            if field.0.as_str() == Some("name")
            {
                job_name = field.1.as_str().unwrap();
            }
            else if field.0.as_str() == Some("parent")
            {
                job_parent = field.1.as_str().unwrap();
                println!("  {} -> {}", job_name, job_parent);
            }
        }
    }
    
    
    // for job in jobs {}
}

// fn parse_deck(doc: &Yaml) -> Vec<Point> {
//     let mut res = Vec::new();
//     match *doc {
//         Yaml::Hash(ref h) => {
//             for (k, v) in h {
//                 match k.as_str() {
//                     Some("jobs") => match v {
//                         Yaml::Hash(ref g) => {
//                             for (nid, xyz) in g {
//                                 let id = nid.as_i64().unwrap();
//                                 let w = match xyz {
//                                     Yaml::Array(ref v) => v,
//                                     _ => panic!("malformed input file: coordinates should be floating point numbers")
//                                 };
//                                 let t = w.iter().map(|x| x.as_f64().unwrap()).collect::<Vec<f64>>();
//                                 let p = Point {id:id, x:t[0], y:t[1], z:t[2]};
//                                 res.push(p);
//                             }
//                         },
//                         _ => panic!("malformed input file: format should be [node-id, x, y, z]")
//                     },
//                     _ =>  panic!("malformed input file: unsuported section.")
//                 };
//             }
//         }
//         _ => {
//             panic!("malformed input file: top level must be in key: value format");
//         }
//     };
//     res
// }
