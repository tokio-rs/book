fn main() {
    let mut doc_files = Vec::new();
    for section in &["../getting-started"] {
        doc_files.extend(skeptic::markdown_files_of_directory(section));
    }

    skeptic::generate_doc_tests(&doc_files);
}
