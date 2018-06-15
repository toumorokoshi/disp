# Disp Runtime

The disp runtime is opinionated, and attempts to achieve a balance of high
performance with a simple programming model.

# In a nutshell

* green threading model with explicit allocation of green threads to one or more
  workers.
* one worker per cpu is recommended
* preemptive scheduling, which can be circumvented with some keyword (atomic? synchronous?)
* shared data across threads uses clojure's atom idea: CAS vs locking of values.

# Green Threading Model

System threads tend to be pretty heavyweight, and optimizations can be made
with regards to the size of the stack, for example.

# Explicit thread assignment to one or more workers.

Workers are system threads, spawned with one per cpu to ensure that multicore
CPUs can be taken advantage of.

Explicit thread assignment is an important detail. By allowing one to dictate
which worker takes on a workload, one can take better advantage of the CPUs L*
cache, by sharing pointers and not moving off of on cpu to another, where a
cache miss would occur.

# Shared data across workers use clojure's atom, CAS-based values.

Ensuring high performance in a concurrent environment is about reducing the
number of locks on data structures. Every lock added results in blocking those
workers from executing.


# Preemptive Scheduling, with the exception of a specific keyword

Preemptive scheduling is a simpler model than explicit async, although one could
add a wait for a future to have state be pollable, or use an epoll signal to notify
when to return operation. Oftentimes these primitives have a right answer on
how to notify the loop of state, so preemptive scheduling can reduce the cognitive
load of async.

To that end, the event loop needs to support:

1. a way to notify based on a kernel-level signal primitive, like epoll or kqueue.
2. a way to return back a future, that can be polled for state.
