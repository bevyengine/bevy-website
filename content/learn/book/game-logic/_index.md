+++
title = "Game logic"
weight = 3
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

Now that you've got the basics of working with the ECS down, let's start thinking about how to structure your game.
While systems are powerful and flexible tools, you still need some higher level structure that tells systems when to run, and allows them to communicate with each other.

The most fundamental tool for this is **scheduling.**
Like in other engines, systems in Bevy are typically run according to a **game loop**, as defined by the {{rust_type(type="struct" crate="bevy_app" name="App")}}'s **runner**.
Each time we go through this game loop, one **tick** is said to have passed, and (usually) one **frame** is drawn to our screen.

The game loop is executed by running the **schedule** on the {{rust_type(type="struct" crate="bevy_ecs" name="World")}}, allowing systems to observe and modify the game state.
The schedule itself is broken down into **stages**, which contain groups of systems that are allowed to execute in a single (typically parallel) block.
Within stages, we can define **explicit system ordering** to ensure that parallel systems run in the order that we need them to.

Once we have a sense of *when* our systems will run, we can start thinking about *if* they will run.
**Run criteria** are granular: they simply define if a system will run during the current stage based on the results of some computation.
**States** are coarse: they allow you to swap between whole groups of systems at a high-level and specify enter and exit logic.
Once you've moved beyond toy examples, you'll find that most systems in most games will belong to one or more states, allowing you to cleanly separate your game and menu code for example.

As your game grows, your logic will naturally start spanning system boundaries.
There are two main options for communicating between systems: modifying entities or resources and sending **events**.
Events should be one of your most common tools: you can to easily pass well-defined information between systems, and as your needs grow you can add new systems that listen for, send and modify these events without touching existing code.

Finally, this chapter covers some other helpful tools to help run and structure your game's logic:

- The {{rust_type(type="struct" crate="bevy_core" name="Timer")}} type allows you to carefully control cooldowns and set delays, while the {{rust_type(type="struct" crate="bevy_core" name="Time")}} resource allows you to compensate for elapsed time correctly.
- **Async tasks** allow you to run long-lived or extremely expensive tasks in a way that won't block the execution of your game loop.
