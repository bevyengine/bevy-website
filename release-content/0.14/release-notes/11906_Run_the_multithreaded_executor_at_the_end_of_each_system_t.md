The largest source of overhead in the multithreaded system executor is from thread context switching.
Each time a thread needs to be woken up it can take up to 30us if the cache for the thread is cold.
So minimizing these switches can be in important optimization for the executor. In this cycle we landed
2 changes that show improments for this.

One of these changes was to run the code that starts new systems after a system completes.
The multithreaded executor is responsible for checking that the dependencies for a system have run already and evaluating the run criteria. If these pass then a task is created for running the system. 
The old version of the multithreaded executor ran as a continuous task that was woken up after each task
completed. This pr changed it so that each system task tries to run the multithreaded executor after each
system completes. This makes it so that the multithreaded executor always runs on a thread that is already awake.
Thus preventing one source of context switches. In practice this reduces the number of context switches per a `Schedule` run by a 1-3. For an improvement of around 30us per schedule. In practice for gpu bound apps, this is probably only a small improvement or potentially a slowdown as there is only one render schedule and this new executor has more overhead for running.

Another change that was done this cycle was to combine all the event_update_system into one system. There was one
instance of this system for each event type. Which ended up with 30+ instances of the system. Each instance ran very quick, and
the overhead of spawning the system tasks and waking up threads to run all the systems dominated the time for the `First` schedule
took to run. So combining all these into one system avoids this overhead and makes the `First` schedule run much faster. On a specific machine this made running the schedule go from 136us to 25us.


