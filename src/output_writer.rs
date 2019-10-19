pub struct OutputWriter {}
impl OutputWriter {
    pub fn prologue() {
        println!("digraph D {{");
    }

    pub fn epilogue() {
        println!("}}");
    }

    pub fn write_output(job_name: &String, job_parent: &String, hex_color: &String) {
        println!("  {} -> {}", job_name, job_parent);
        println!(
            "  \"{}\" [style=filled, fillcolor=\"{}\"]",
            job_name, hex_color
        );
    }
}
