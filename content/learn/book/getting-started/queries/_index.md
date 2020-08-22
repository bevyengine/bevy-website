+++
title = "Queries"
weight = 6
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

In Bevy ECS, `Queries` give you direct control over entity iteration. They also provide a few extra filtering options, such as `With<Component>` and `Without<Component>` filters.

## Fixing our Bug

Up until now, we have been using **For-Each Systems**, which directly reference components and run once for each entity. In the case of our `greet_people` system, the **For-Each System** behavior doesn't quite line up with the logic we want because it runs once for each entity:

```rs
fn greet_people(
    time: Res<Time>, mut timer: ResMut<GreetTimer>, person: &Person, name: &Name) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        println!("hello {}!", name.0);
    }
}
```

The solution is to use a **Query System** instead:

```rs
fn greet_people(
    time: Res<Time>, mut timer: ResMut<GreetTimer>, mut query: Query<(&Person, &Name)>) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        for (_person, name) in &mut query.iter() {
            println!("hello {}!", name.0);
        }
    }
}
```

Now our App should properly greet all `People`, once every two seconds!


## Query Systems vs For-Each Systems

Now that we have two different system types, how do you choose which one to use?

This is largely an aesthetic question. Some developers will prefer the simplicity and legibility of **For-Each Systems** and will use them wherever they can. Others will prefer the flexibility of **Query Systems**. There are no right answers here. You should use what works best for you!