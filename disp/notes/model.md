* lightweight threading model

# Actors?

The main issue is how to protect against race conditions. How?

Race conditions comes from data depending on each other. A -> B, B -> A.

async programming (explicitly specifying what is block and what is non-blocking) is expensive, and it's great to be able to preempt as necessary.
