The largest source of overhead in Bevy's multithreaded system executor is from
[thread context switching](https://en.wikipedia.org/wiki/Context_switch), i.e. starting and stopping threads.
Each time a thread is woken up it can take up to 30us if the cache for the thread is cold.
Minimizing these switches is an important optimization for the executor. In this cycle we landed
two changes that show improveoments for this.

One of these changes was to run the executor for starting system tasks in each task for running a system.
The system executor is responsible for checking that the dependencies for a system have run already
and evaluating the run criteria and then running a task for that system.
The old version of the multithreaded executor ran as a separate task that was woken up after each task
completed. This would sometimes cause a new thread to be woken up for the executor to process the system completing.
This pr changed it so that each system task tries to run the multithreaded executor after each system completes.
This makes it so that the multithreaded executor always runs on a thread that is already awake.
Thus preventing one source of context switches. In practice this reduces the number of context switches per a `Schedule` run by 1-3 times.
For an improvement of around 30us per schedule. In practice for gpu bound apps, this may only a small improvement, since there is only one schedule for the render app. But in CPU bound situation this may be a significant gain as the main schedules has many schedules that it runs.

The other change to reduce executor overhead was to combine all the `event_update_system`'s into one system. There was one
instance of this system for each event type, which with just the `DefaultPlugins` ended up with 20+ instances of the system.
Each instance ran very quick, so the overhead of spawning the system tasks and waking up threads to run all these
systems dominated the time it took for the `First` schedule to run. So combining all these into one system avoids this overhead and makes the `First` schedule run much faster. In testing this made running the schedule go from 140us to 25us.
