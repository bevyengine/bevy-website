use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

use crate::{
    code_block_definition::CodeBlockDefinition, hidden_ranges::get_hidden_ranges,
    utils::visit_dir_md_files,
};

pub fn run(dir: &Path) {
    println!("Formatting folder: {:?}", dir);

    let result = visit_dir_md_files(dir, &|entry| {
        println!("{:?}", entry.path());

        let file = File::open(entry.path())?;
        let file_len = file.metadata().unwrap().len().try_into().unwrap();
        let mut contents = String::with_capacity(file_len);
        let mut is_inside_rust_code_block = false;
        let mut rust_block: Vec<String> = vec![];

        for line in io::BufReader::new(file).lines() {
            let line = line?;
            let is_code_block_open = line.starts_with("```rust");
            let is_code_block_close = line == "```";

            if is_inside_rust_code_block && is_code_block_open {
                panic!("Nested '```rust' code block not allowed");
            } else if is_code_block_open {
                is_inside_rust_code_block = true;
            }

            // Skip the line, save it as is
            if !is_inside_rust_code_block {
                contents.push_str(&format!("{}\n", &line));
                continue;
            }

            rust_block.push(line);

            // Process the `rust` code block
            if is_code_block_close {
                let code = &rust_block[1..rust_block.len() - 1];
                let real_hidden_ranges = get_hidden_ranges(code);
                let mut definition = CodeBlockDefinition::new(&rust_block[0]).unwrap();

                match definition.get_hidden_ranges() {
                    Some(annotation_hidden_ranges) => {
                        if *annotation_hidden_ranges != real_hidden_ranges {
                            definition.set_hidden_ranges(real_hidden_ranges);
                        }
                    }
                    None => {
                        if !real_hidden_ranges.is_empty() {
                            definition.set_hidden_ranges(real_hidden_ranges);
                        }
                    }
                }

                // Rewrite code block Zola annotations
                rust_block[0] = definition.into_string();

                // Write code block
                contents.push_str(&format!("{}\n", &rust_block.join("\n")));

                // Reset state
                is_inside_rust_code_block = false;
                rust_block = vec![];
            }
        }

        // Rewrite file
        fs::write(entry.path(), contents)?;

        Ok(())
    });

    match result {
        Ok(_) => println!("Done!"),
        Err(error) => println!("Error: {}", error),
    }
}
