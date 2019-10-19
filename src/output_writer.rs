pub struct OutputWriter {}
impl OutputWriter {
    pub fn prologue() -> String{
        "digraph D {\n".to_string()
    }

    pub fn epilogue() -> String{
        "}\n".to_string()
    }

    pub fn write_output(job_name: &String, job_parent: &String, hex_color: &String) -> String {
        let mut result = format!("  {} -> {}\n", job_name, job_parent);
        result += &format!("  \"{}\" [style=filled, fillcolor=\"{}\"]\n", job_name, hex_color);
        result
    }
}
