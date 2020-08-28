+++
title = "ecs tutorial"
weight = 3
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "left"
+++

<div style="text-align:right;"><img src="/assets/bevy_tutor_icon.svg" style="height: 4em;" alt="BEVY"/></div>

This tutorial aims to give both an awareness and practical examples of the following:
* There is a startup scheduler and a normal scheduler ({{rust_type(type="struct", crate="bevy_ecs", name="Schedule", no_mod=true, plural=false)}})
* Startup systems run only ONCE before normal systems
* You can ask normal systems to run once, or loop for `k` seconds
  - where `k ∈ [1, 9223372036854775807_i64]` seconds (i.e. ~292 billion years)
* A **System** results from calling the inline {{rust_type(type="trait" crate="bevy_ecs", name="IntoForEachSystem" method="system" no_struct=false)}} method on a `fn`. It is a {{rust_type(type="struct", crate="bevy_ecs", name="SystemFn", no_mod=true, plural=false)}} struct that implements the {{rust_type(type="trait", crate="bevy_ecs", name="System", no_mod=true, plural=false)}} trait and encapsulates the original function 
* A **System** is always assigned a **stage** in the scheduler
  - This dictates the group of systems that it runs in parallel with
  - The default normal stage is `stage::UPDATE`
  - The default startup stage is `startup_stage::STARTUP`
* Except when certain data constraints exist (i.e. thread local with exclusive access to {{rust_type(type="struct", crate="bevy_ecs", name="World", no_mod=true, plural=false)}} and {{rust_type(type="struct", crate="bevy_ecs", name="Resources", no_mod=true, plural=false)}})
  - Startup systems run in no particular order within their **stage** due to parallel execution
  - Normal systems run in no particular order within their **stage** due to parallel execution
* Stages exist as an ordered collection
* For finer control of the order of execution within a scheduler
  - You can create as many NEW stages in the scheduler as you like
  - New stages are added immediately *before* or *after* an existing stage (system defined or custom)
* You can emit and consume custom events
* You can add new **Components** and **Systems** to a running {{rust_type(type="struct", crate="bevy_app", name="App", no_mod=true, plural=false)}}, i.e. after `.run()`
  - with `commands:` {{rust_type(type="struct", crate="bevy_ecs", name="Commands", no_mod=true, plural=false)}}
* {{rust_type(type="struct", crate="bevy_app", name="App", no_mod=true, plural=true)}} also have a mechanism for exiting gracefully

## Create Project

First [create](@/learn/book/getting-started/setup/_index.md#create-a-new-rust-executable-project) a new project called `bevy_ecs_tut` and [add](@/learn/book/getting-started/setup/_index.md#add-bevy-to-your-project-s-cargo-toml) <img src="/assets/bevy_logo_dark.svg" style="height: 1em;margin-bottom:-0.1em" alt="BEVY"/> as a dependency.

If you don't know or remember the current version of Bevy, `cargo` can fetch it for you fast.
```bash
$ cargo search bevy
bevy = "0.1.3"                  # A refreshingly simple data-driven ...
```
<p class="bevy-figure-caption">3.1: Truncated output of cargo search</p>

## AppBuilder

<p class="bevy-code-filename">src/main.rs</p>

```rs
use bevy::prelude::*;

fn main() {
    App::build()
        .init_resource::<GameState>()
        .add_startup_system(print_initial_system.system())
        .run();
}

#[derive(Default)]
struct GameState {
    current_round: usize,
    total_players: usize,
    winning_player: Option<String>,
}

fn print_initial_system(state: Res<GameState>) {
    println!("Welcome to the Game!
The default GameState is:\n\tCurrent Round: {}
\tTotal Players: {}\n\tWinning Player: {:?}",
          state.current_round, state.total_players, state.winning_player);
}
```
<p class="bevy-figure-caption">3.2: A simple App</p>

At the risk of stating the obvious.
```bash
$ cargo run

Welcome to the Game!
The default GameState is:
        Current Round: 0
        Total Players: 0
        Winning Player: None
```
<p class="bevy-figure-caption">3.3: First game output</p>

The values of `GameState` come from the derived `Default` implementation.

{{rust_type(type="struct" crate="bevy_app", name="App" method="build" no_struct=false)}} makes an {{rust_type(type="struct", crate="bevy_app", name="AppBuilder", no_mod=true, plural=false)}} for us, on which we can call inline methods to add **Systems**, **Components**, and configuration.

{{rust_type(type="struct" crate="bevy_app", name="AppBuilder" method="init_resource" no_struct=false)}} inserts `GameState` into {{rust_type(type="struct", crate="bevy_app", name="App", no_mod=true, plural=false)}}::resources<{{rust_type(type="struct" crate="bevy_ecs", name="Resources", no_mod=false, plural=false)}}>, making `GameState` a **Component**.

It can do this because `GameState` derives `Default` and because we anotate the type for the compiler.
We could have done this with {{rust_type(type="struct" crate="bevy_app", name="AppBuilder" method="add_resource" no_struct=false)}}.

Then {{rust_type(type="struct" crate="bevy_app", name="AppBuilder" method="add_startup_system" no_struct=false)}} takes the system created with {{rust_type(type="trait" crate="bevy_ecs", name="IntoForEachSystem" method="system" no_struct=true)}} and attaches it at the `startup_stage::STARTUP` stage.

Internally, `.add_startup_system(print_initial_system.system())`
is called as `.add_startup_system_to_stage(startup_stage::POST_STARTUP, print_initial_system.system())` ({{rust_type(type="struct" crate="bevy_app", name="AppBuilder" method="add_startup_system_to_stage" no_struct=true)}}).

### Startup Schedule

A startup scheduler runs only once and we can add systems to it with {{rust_type(type="struct" crate="bevy_app", name="AppBuilder" method="add_startup_system" no_struct=true)}} or {{rust_type(type="struct" crate="bevy_app", name="AppBuilder" method="add_startup_system_to_stage" no_struct=true)}}.
The startup scheduler only has two in-built stages, `startup_stage::STARTUP` and `startup_stage::POST_STARTUP`.

### Normal Schedule

Systems assigned to the normal scheduler can run one or more times depending on whether we add a looping mechanism to the App.
They are added to the App with {{rust_type(type="struct" crate="bevy_app", name="AppBuilder" method="add_system" no_struct=true)}} or {{rust_type(type="struct" crate="bevy_app", name="AppBuilder" method="add_system_to_stage" no_struct=true)}}. The normal stages are `stage::{FIRST, EVENT_UPDATE, PRE_UPDATE, UPDATE, POST_UPDATE, LAST}`.


For both normal and startup there are two functions for adding new stages, either before or after an existing stage. You'll see them soon.
You can add as many stages as you need because they are stored in an ordered collection. I.e. if you added ten stages before `stage::UPDATE` none of them would ever run before `stage::PRE_UPDATE`.
