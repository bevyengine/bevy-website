+++
title = "This is a Test Post"
date = 2020-04-02
[extra]
author = "Carter Anderson"
image = "mountains.png"
+++

This is a test post. You somehow found this website before Bevy was announced. Congrats!

<!-- more -->

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse commodo laoreet quam. Pellentesque imperdiet lorem a urna eleifend molestie. Proin pharetra cursus nunc, quis luctus nisi molestie eu. In a lobortis eros. Aenean sagittis urna at fermentum luctus. Ut odio eros, aliquam ac dui sit amet, efficitur tincidunt dolor. Praesent quis turpis felis. Etiam quis porta neque. Curabitur egestas egestas ipsum vel varius.
Sed tincidunt est pretium enim gravida tincidunt. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Phasellus facilisis quam lectus, a venenatis urna commodo id. Interdum et malesuada fames ac ante ipsum primis in faucibus.

Suspendisse in tortor interdum, aliquam odio vel, faucibus quam. Sed urna justo, malesuada et tortor at, fringilla volutpat urna. Pellentesque a hendrerit felis, a fermentum mauris. Nullam rhoncus nibh nec sapien rutrum convallis. Mauris condimentum sapien consequat neque pretium mattis. Nunc elementum sodales neque non tincidunt. Suspendisse eget elit non justo lobortis lobortis. Ut eget ornare urna. Aliquam quis risus in mi eleifend eleifend. Vivamus lobortis aliquam sem sit amet rhoncus. Sed ornare vulputate turpis, ac volutpat dolor tempus eget. Nam maximus arcu orci. Nam lobortis nisi non urna venenatis, quis lobortis nisl suscipit. Fusce convallis, elit non mattis finibus, tortor leo scelerisque arcu, et egestas tellus magna vel lacus. 

```rs
use bevy::core::event::Events;

struct MyEvent {
    value: usize
}
// setup
let mut events = Events::<MyEvent>::default();
let mut reader = events.get_reader();

// run this once per update/frame
events.update();

// somewhere else: send an event
events.send(MyEvent { value: 1 });

// somewhere else: read the events
for event in events.iter(&mut reader) {
    assert_eq!(event.value, 1)
}

// events are only processed once per reader
assert_eq!(events.iter(&mut reader).count(), 0);
```

 Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse commodo laoreet quam. Pellentesque imperdiet lorem a urna eleifend molestie. Proin pharetra cursus nunc, quis luctus nisi molestie eu. In a lobortis eros. Aenean sagittis urna at fermentum luctus. Ut odio eros, aliquam ac dui sit amet, efficitur tincidunt dolor. Praesent quis turpis felis. Etiam quis porta neque. Curabitur egestas egestas ipsum vel varius. Sed tincidunt est pretium enim gravida tincidunt. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Phasellus facilisis quam lectus, a venenatis urna commodo id. Interdum et malesuada fames ac ante ipsum primis in faucibus.

Suspendisse in tortor interdum, aliquam odio vel, faucibus quam. Sed urna justo, malesuada et tortor at, fringilla volutpat urna. Pellentesque a hendrerit felis, a fermentum mauris. Nullam rhoncus nibh nec sapien rutrum convallis. Mauris condimentum sapien consequat neque pretium mattis. Nunc elementum sodales neque non tincidunt. Suspendisse eget elit non justo lobortis lobortis. Ut eget ornare urna. Aliquam quis risus in mi eleifend eleifend. Vivamus lobortis aliquam sem sit amet rhoncus. Sed ornare vulputate turpis, ac volutpat dolor tempus eget. Nam maximus arcu orci. Nam lobortis nisi non urna venenatis, quis lobortis nisl suscipit. Fusce convallis, elit non mattis finibus, tortor leo scelerisque arcu, et egestas tellus magna vel lacus. 