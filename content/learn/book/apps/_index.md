+++
title = "Apps"
weight = 4
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

Bevy programs are referred to as {{rust_type(type="struct", mod="bevy::app", name="App", short=true, plural=true)}}. The simplest Bevy app looks like this:

```rs
use bevy::prelude::*;

fn main() {
    App::build().run();
}
```

Nice and simple right? Copy the code above into your ```main.rs``` file. Then run

```cargo run```

in your project folder. You will notice that ... nothing happens. This is because we haven't told our app to do anything yet!

Apps are just empty shells capable of running our application logic. In the next section, we will add logic using "app plugins".