use std::{env, path::Path};

use write_rustdoc_hide_lines::formatter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let folder = args.get(1).expect("Please provide a folder");
    let path = Path::new(&folder);

    println!("Formatting folder: {:?}", path);

    match formatter::run(path) {
        Ok(_) => println!("Done!"),
        Err(error) => println!("Error: {}", error),
    }
}
