* split out VMFunction and NativeFunction, as otherwise
  they will not fit into 64 bit registers in the VM.
* make different instructions for each, and handle them
  differently.
