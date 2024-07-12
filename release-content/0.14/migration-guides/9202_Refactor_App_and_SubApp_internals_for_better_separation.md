`SubApp` has been separated from `App`, so there are a few larger changes involved when interacting with these types.

#### Constructing a `SubApp`

`SubApp` no longer contains an `App`, so you no longer are able to convert an `App` into a `SubApp`. Furthermore, the extraction function must now be set outside of the constructor.

```rust
// 0.13
#[derive(AppLabel, Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct MySubApp;

let mut app = App::new();
let mut sub_app = App::empty();

sub_app.add_systems(Main, ...);
sub_app.insert_resource(...);

app.insert_sub_app(MySubApp, SubApp::new(sub_app, |main_world, sub_app| {
    // Extraction function.
}));

// 0.14
#[derive(AppLabel, Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct MySubApp;

let mut app = App::new();
// Use `SubApp::new()` instead of `App::new()`.
let mut sub_app = SubApp::new();

// Instead of setting the extraction function when you create the `SubApp`, you must set it
// afterwards. If you do not set an extraction function, it will do nothing.
sub_app.set_extract(|main_world, sub_world| {
    // Extraction function.
});

// You can still add systems and resources like normal.
sub_app.add_systems(Main, ...);
sub_app.insert_resource(...);

app.insert_sub_app(MySubApp, sub_app);
```

#### `App` changes

`App` is not `Send` anymore, but `SubApp` still is.

Due to the separation of `App` and `SubApp`, a few other methods have been changed.

First, `App::world` as a property is no longer directly accessible. Instead use the getters `App::world` and `App::world_mut`.

```rust
#[derive(Component)]
struct MyComponent;

// 0.13
let mut app = App::new();
println!("{:?}", app.world.id());
app.world.spawn(MyComponent);

// 0.14
let mut app = App::new();
println!("{:?}", app.world().id()); // Notice the added paranthesese.
app.world_mut().spawn(MyComponent);
```

Secondly, all getters for the sub app now return a `SubApp` instead of an `App`. This includes `App::sub_app`, `App::sub_app_mut`, `App::get_sub_app`, and `App::get_sub_app_mut`.

```rust
#[derive(AppLabel, Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct MySubApp;

let mut app = App::new();
app.insert_sub_app(MySubApp, SubApp::new());

assert_eq!(app.sub_app(MySubApp).type_id(), TypeId::of::<SubApp>());
```

Finally, `App::runner` and `App::main_schedule_label` are now private. It is no longer possible to access the runner function, but you can access the main schedule label using `SubApp::update_schedule`.

```rust
let app = App::new();
let label = app.main().update_schedule;
```

#### 3rd-party traits on `App`

If you implemented an extension trait on `App`, consider also implementing it on `SubApp`:

```rust
trait SpawnBundle {
    /// Spawns a new `Bundle` into the `World`.
    fn spawn_bundle<T: Bundle>(&mut self, bundle: T) -> &mut Self;
}

impl SpawnBundle for App {
    fn spawn_bundle<T: Bundle>(&mut self, bundle: T) -> &mut Self {
        self.world_mut().spawn(bundle);
        self
    }
}

/// `SubApp` has a very similar API to `App`, so the code will usually look the same.
impl SpawnBundle for SubApp {
    fn spawn_bundle<T: Bundle>(&mut self, bundle: T) -> &mut Self {
        self.world_mut().spawn(bundle);
        self
    }
}
```
