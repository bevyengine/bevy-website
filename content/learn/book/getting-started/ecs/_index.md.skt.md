```rust,skt-main
use bevy::prelude::*;

fn main() {{
    {}
}}
```

```rust,skt-system-print_position_system
use bevy::prelude::*;

fn main() {{
    {}
    print_position_system.system();
}}
```

```rust,skt-system-hello_world
use bevy::prelude::*;

fn main() {{
    {}
    hello_world.system();
}}
```

```rust,skt-system-add_people
use bevy::prelude::*;

struct Person;
struct Name(String);

fn main() {{
    {}
    add_people.system();
}}
```

```rust,skt-system-greet_people
use bevy::prelude::*;

struct Person;
struct Name(String);

fn main() {{
    {}
    greet_people.system();
}}
```

```rust,skt-import
use bevy::prelude::*;

fn hello_world() {{}}
fn add_people() {{}}
fn greet_people() {{}}

{}
```
