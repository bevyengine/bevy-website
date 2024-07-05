The largest source of overhead in Bevy's multithreaded system executor is from
[thread context switching](https://en.wikipedia.org/wiki/Context_switch), i.e. starting and stopping threads.
Each time a thread is woken up it can take up to 30us if the cache for the thread is cold.
Minimizing these switches is an important optimization for the executor. In this cycle we landed
two changes that show improvements for this:

#### Run the multi-threaded executor at the end of each system task

The system executor is responsible for checking that the dependencies for a system have run already
and evaluating the run criteria and then running a task for that system.
The old version of the multithreaded executor ran as a separate task that was woken up after each task
completed. This would sometimes cause a new thread to be woken up for the executor to process the system completing.

By changing it so the system task tries to run the multithreaded executor after each system completes, we ensure that the multithreaded executor always runs on a thread that is already awake. This prevents one source of context switches. In practice this reduces the number of context switches per a `Schedule` run by 1-3 times, for an improvement of around 30us per schedule. When an app has many schedules, this can add up!

#### Combined Event update system

There used to be one
instance of the "event update system" for each event type. With just the `DefaultPlugins`, that results in 20+ instances of the system.

Each instance ran very quick, so the overhead of spawning the system tasks and waking up threads to run all these systems dominated the time it took for the `First` schedule to run. So combining all these into one system avoids this overhead and makes the `First` schedule run much faster. In testing this made running the schedule go from 140us to 25us. Again, not a *huge* win, but we're all about saving every microsecond we can!
