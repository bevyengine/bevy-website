# Learning Code Examples

This folder contains all the examples used in the official Bevy website learning materials, e.g., Bevy Book, Quick Start Guide, Advanced Examples, et cetera.

## Usage

1. Create an `*.rs` example in `./examples/{LEARNING_MATERIAL}` where `{LEARNING_MATERIAL}` is the official Bevy learning material the example will be used in, e.g., Bevy Book, Quick Start Guide, Advanced Examples, et cetera.
  > [!TIP]
  > In these Rust examples you use `// ANCHOR: name` and `// ANCHOR_END: name` to define the snippets that can be used in the pages. These anchors can also be nested; they won't parse the ANCHOR instructions other than the one that is intended to be parsed. You also have `// HIDE` which you can append to the end of a line to hide that line from being parsed / rendered.

  ```rs
  // ANCHOR: full_program
  use bevy::prelude::*;

  // ANCHOR: basic_component
  #[derive(Component)]
  struct BasicComponent;
  // ANCHOR_END: basic_component

  // ANCHOR: basic_system
  fn basic_system(mut commands: Commands) {
    commands.spawn(BasicComponent);
    eprintln!("spawning Basic Component"); // HIDE
  }
  // ANCHOR_END: basic_system

  // ANCHOR: app_main
  fn main() {
    App::new().add_systems(Startup, basic_system).run();
    eprintln!("app ran"); // HIDE
  }
  // ANCHOR_END: app_main
  // ANCHOR_END: full_program
  ```

2. Add it to `Cargo.toml` with the format:

  ```toml
  [[example]]
  name = "kebab-case-name-of-example"
  path = "examples/{LEARNING_MATERIAL}/snake_case_name_of_example.rs"
  ```

3. Use the `{{file_code_block(file="{LEARNING_MATERIAL}/{FILE}.rs", anchor="{ANCHOR}")}}` shortcode in the page you wish to embed the code example / block.

> [!NOTE]
> The file in the `file_code_block` shortcode is already prefixed with `learning-code-examples/examples`!

## Validating Code

To validate code you can run the bash script contained in this directory:
`./validate_examples.sh`

>[!TIP]
> The bash script can be called from any directory!

However, if you can't run the bash script for one reason or another then you can run `cargo check --examples && cargo clippy --examples && cargo fmt --check` _in this directory_.
