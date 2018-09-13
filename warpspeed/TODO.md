* split out VMFunction and NativeFunction, as otherwise
  they will not fit into 64 bit registers in the VM.
* make different instructions for each, and handle them
  differently.
* Ensure that the worker heap is designed for optimal cache affinity. We might have a lot of leaking of variable size data structures such as strings and maps leaking to all over the heap, which may or may not be a problem.
