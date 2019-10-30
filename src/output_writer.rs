pub struct OutputWriter {}
impl OutputWriter {
    pub fn prologue() -> String {
        let mut result = format!("digraph D {{\n  rankdir=RL;\n");
        result += format!("  node[shape=record]\n").as_str();
        result.to_string()
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

    pub fn write_output(
        job_name: &String,
        job_parent: &String,
        attributes: &Vec<String>,
        hex_color: &String,
    ) -> String {
        if job_parent == "" {
            return format!("");
        }

        let mut result = format!("  \"{}\" -> \"{}\"\n", job_name, job_parent);
        result += &format!(
            "  \"{}\" [style=filled, fillcolor=\"{}\"",
            job_name, hex_color
        );

        if attributes.len() > 0 {
            result += format!(", label=\"{{ {} | ", job_name).as_str();
            for attribute in attributes.iter() {
                result += format!("{}\\n ", attribute).as_str();
            }
            result += format!("}}\"").as_str();
        }

        result += &format!("]\n");
        result
    }
}
