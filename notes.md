#keep.google.com

https://internals.rust-lang.org/t/pre-rfc-thread-affinity/3117/9

# Tokio Stack

* run() spawns a Runtime
* Runtime.spawn() sends a future to the Pool
* Pool uses tokio_executor::Executor
  * pool can be passed the list of worker entries, via Pool::new()
  * implicitly uses num_cpus, so we probably don't need to do more than that.
    although maybe we should for thread pinning.
  * isn't an easy way exposed in Tokio. Tokio's default ThreadPool spawns
    via an unconfigured builder.
* Pool.submit submits to a random worker
* entry.submit_external will submit to the
  entry queue, and will fail only if the
  worker is shutting down.
* if submit_external fails, a worker will be
  spawned with that worker id, ensuring that the
  recently enqueued entry into the queue
  will be taken up by a new worker.
