# The Design of Warpspeed

## The Worker Model

Unlike languages that completely abstract away the concept of threads using fibers,
warpspeed exposes this via the concept of workers.

At the start, and by default, warpspeed creates N worker threads, one per CPU (or hyperthread). These worker threads will then be proceeded to be pinned to each available CPU, if possible.

By pinning a worker to a CPU, it ensures that one can take advantage of cache locality. By specifying a specific CPU on which a fiber should run, it also better ensures cache locality.
