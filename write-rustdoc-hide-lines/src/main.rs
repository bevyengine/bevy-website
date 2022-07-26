use std::{env, path::Path};

use write_rustdoc_hide_lines::formatter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let folder = args.get(1);

    if let Some(folder) = folder {
        let path = Path::new(&folder);
        formatter::run(path);
    } else {
        panic!("Please provide a folder.");
    }
}
