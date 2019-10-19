use crate::output_writer::OutputWriter;
use crate::random_color_generator::RandomColorGenerator;
use crate::yaml_parser::YamlParser;

pub struct DotGenerator {}
impl DotGenerator {
    pub fn run(&self, args: Vec<String>) -> String{
        let yaml_parser = YamlParser {};
        let mut result : String;
        result = OutputWriter::prologue();

        for file in &args[1..] {
            let jobs = yaml_parser.get_tags(&file, "jobs");
            result += &self.generate_dot(jobs);
        }

        result += &OutputWriter::epilogue();
        result
    }

    fn generate_dot(&self, jobs: yaml_rust::Yaml) -> String{
        let mut result : String = "".to_string();

        let hexified_rgb = RandomColorGenerator::get_random_color().to_owned();
        let hex_color = "#".to_owned() + &hexified_rgb;

        let mut job_name: String = "".to_string();
        let mut job_parent: String = "".to_string();

        for job in jobs.as_vec().unwrap() {
            for field in job.as_hash().unwrap() {
                result += &self.generate_node_and_edge(field, &hex_color, &mut job_name, &mut job_parent);
            }
        }
        result
    }

    fn generate_node_and_edge(
        &self,
        field: (&yaml_rust::yaml::Yaml, &yaml_rust::yaml::Yaml),
        hex_color: &String,
        job_name: &mut String,
        job_parent: &mut String,
    ) -> String {
        let mut result : String = "".to_string();
        if field.0.as_str() == Some("name") {
            *job_name = field.1.as_str().unwrap().to_owned();
        } else if field.0.as_str() == Some("parent") {
            *job_parent = field.1.as_str().unwrap().to_owned();
            result = OutputWriter::write_output(job_name, job_parent, hex_color);
        }
        result
    }
}
