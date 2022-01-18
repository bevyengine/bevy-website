+++
title = "Apps"
weight = 2
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
+++

Bevy programs are referred to as {{rust_type(type="struct", crate="bevy_app", name="App", no_mod=true, plural=true)}}. The simplest Bevy app looks like this:

```rs
use bevy::prelude::*;

fn main() {
    App::new().run();
}
```

Nice and simple right? Copy the code above into your ```main.rs``` file, then run:

```sh
cargo run
```

in your project folder. You will notice that ... nothing happens. This is because we haven't told our app to do anything yet! Apps are just empty shells capable of running our application logic. Let's add some logic to our App using Bevy ECS.
