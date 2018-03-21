# Process Supervision
* if an actor dies, it will be revived
* green threading model to handle actors
* message passing to communicate with actors
    * message passing ensures data isolation
* how to handle communication between multiple processes?
    * write queue, no delay reads
* stream processing works great for web applications


# Problems to solve with this model

* if channels are the way to defend against locking, it's not a freebie. It
must be architected as a conscious choice. This makes refactoring in this area hard.
