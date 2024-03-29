use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlLoader;

pub struct YamlParser {}
impl YamlParser {
    pub fn get_tags(&self, file: &str, tag: Option<String>) -> yaml_rust::Yaml {
        let doc = self.load_file(file);
        match tag {
            None => doc,
            Some(extract_tag) => doc[&extract_tag[..]].clone(),
        }
    }
    fn load_file(&self, file: &str) -> yaml_rust::Yaml {
        let mut file = File::open(file).expect("Unable to open file");
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .expect("Unable to read file");

        let docs = YamlLoader::load_from_str(&contents).unwrap();
        docs[0].clone()
    }
}
