`bevy::ecs::IntoSystemConfigs`, now known as `IntoScheduleConfigs`, is no longer implemented for `BoxedSystem<(), ()>`. This can lead to convoluted trait errors when you try to add a `BoxedSystem<(), ()>` to a schedule or app:

```
error[E0277]: `std::boxed::Box<dyn bevy::prelude::System<In = (), Out = ()>>` does not describe a valid system configuration
```

In order to avoid this error, either wrap your system in an `InfallibleSystemWrapper` before boxing it or make the system return a `Result<(), BevyError>`.

```rust
// 0.15
fn my_system() {
    println!("Hello, world!");
}

// Convert the function into a boxed system, which is a `Box<dyn System<In = (), Out = ()>>`.
let system = Box::new(IntoSystem::into_system(my_system)) as BoxedSystem;

App::new()
    .add_systems(Startup, system)
    .run();

// 0.16 (Using `InfallibleSystemWrapper`)
fn my_system() {
    println!("Hello, world!");
}

// Use `InfallibleSystemWrapper::new()` to make a system unconditionally return `Result::Ok`. The
// boxed system is now a `Box<dyn System<In = (), Out = Result<(), BevyError>>>`.
let system = Box::new(InfallibleSystemWrapper::new(IntoSystem::into_system(my_system))) as BoxedSystem<_, _>;

App::new()
    .add_systems(Startup, system)
    .run();

// 0.16 (Returning `Result<(), BevyError>`)
fn my_system() -> Result {
    println!("Hello, world!");
    Ok(())
}

// The boxed system is now a `Box<dyn System<In = (), Out = Result<(), BevyError>>>`.
let system = Box::new(IntoSystem::into_system(my_system)) as BoxedSystem<_, _>;

App::new()
    // Add the boxed system to the app.
    .add_systems(Startup, system)
    .run();
```

Note that in several cases you do not need to box your systems before adding them, such as with `App::add_systems()`, which lets you avoid this issue.
