```skt-main,rust
use bevy::prelude::*;

fn main() {{
    {}
}}
```

```skt-system-print_position_system,rust
use bevy::prelude::*;

fn main() {{
    {}
    print_position_system.system();
}}
```

```skt-system-hello_world,rust
use bevy::prelude::*;

fn main() {{
    {}
    hello_world.system();
}}
```

```skt-system-add_people,rust
use bevy::prelude::*;

struct Person;
struct Name(String);

fn main() {{
    {}
    add_people.system();
}}
```

```skt-system-greet_people,rust
use bevy::prelude::*;

struct Person;
struct Name(String);

fn main() {{
    {}
    greet_people.system();
}}
```

```skt-import,rust
use bevy::prelude::*;

fn hello_world() {{}}
fn add_people() {{}}
fn greet_people() {{}}

{}
```
