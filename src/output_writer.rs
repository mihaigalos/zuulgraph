pub struct OutputWriter {}
impl OutputWriter {
    pub fn prologue() -> String {
        "digraph D {\n  rankdir=RL;\n".to_string()
    }

    pub fn epilogue() -> String {
        "}\n".to_string()
    }

    pub fn write_legend(files: Vec<String>, colors: Vec<String>) -> String {
        let mut result = format!("  subgraph cluster_01 {{\n    label = \"Legend\";\n");
        for i in 0..files.len() {
            result += format!(
                "    \"{}\" [style=filled, fillcolor=\"{}\"]\n",
                files[i], colors[i]
            )
            .as_str();
        }
        result += format!("  }}\n").as_str();
        result
    }

    pub fn write_output(job_name: &String, job_parent: &String, hex_color: &String) -> String {
        let mut result = format!("  \"{}\" -> \"{}\"\n", job_name, job_parent);
        result += &format!(
            "  \"{}\" [style=filled, fillcolor=\"{}\"]\n",
            job_name, hex_color
        );
        result
    }
}
