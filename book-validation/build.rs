use skeptic::*;

fn main() {
    let mdbook_files = markdown_files_of_directory("../content/learn/book/getting-started")
        .into_iter()
        .filter(|path| {
            !path
                .as_os_str()
                .to_string_lossy()
                // exclude migration guides as it will contain invalid
                // code for current version of Bevy
                .contains("migration-guides")
        })
        .collect::<Vec<_>>();
    generate_doc_tests(&mdbook_files);
}
