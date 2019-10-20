#[cfg(test)]
mod tests {
    use crate::dot_generator::DotGenerator;
    use regex::Regex;

    #[test]
    fn test_when_typical() {
        let dot_generator = DotGenerator {};
        let args = [
            "zuulgraph",
            "/home/mihai/git/zuulgraph/test/yaml/demo_file1.yaml",
            "/home/mihai/git/zuulgraph/test/yaml/demo_file2.yaml",
        ]
        .iter()
        .map(|&s| s.into())
        .collect();
        let expected = concat!(
            "digraph D {\n",
            "  JobB_file1 -> A_file2\n",
            "  \"JobB_file1\" [style=filled, fillcolor=\"\"]\n",
            "  JobC_file1 -> JobB_file1\n",
            "  \"JobC_file1\" [style=filled, fillcolor=\"\"]\n",
            "  JobD_file1 -> JobB_file1\n",
            "  \"JobD_file1\" [style=filled, fillcolor=\"\"]\n",
            "  A_file2 -> JobA_file1\n",
            "  \"A_file2\" [style=filled, fillcolor=\"\"]\n",
            "}\n"
        );

        let result = dot_generator.run(args);
        let re = Regex::new(r"#[A-Z0-9]+").unwrap();
        let postprocessed_result = re.replace_all(result.as_str(), "");

        assert_eq!(postprocessed_result, expected);
    }
}
