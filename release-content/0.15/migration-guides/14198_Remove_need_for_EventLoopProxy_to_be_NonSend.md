`EventLoopProxy` has been renamed to `EventLoopProxyWrapper` and is now `Send`, making it an ordinary resource.

Before:

```rust
event_loop_system(event_loop: NonSend<EventLoopProxy<MyEvent>>) {
    event_loop.send_event(MyEvent);
}
```

After:

```rust
event_loop_system(event_loop: Res<EventLoopProxy<MyEvent>>) {
    event_loop.send_event(MyEvent);
}
```
