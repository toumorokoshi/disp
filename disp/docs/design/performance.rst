Design Choices that Impact Performance
======================================

Fibers Instead of Threads
*************************

System threads work well to distribute CPU-bound workloads, but are expensive when used in a 1-1 ratio with network requests.

Newer programming languages and paradigms have introduced the idea of an eventloop and/or green threading: lightweight threads that are multiplexed on a single thread. These lightweight threads can use operating system level constructs to handle networking, similar to classic threads. The benefits are:

* memory: threads implemented in the application do not require the full stack that system threads too, and thus can be smaller.
*
