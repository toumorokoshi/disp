# implementation

## Compile time execution

Some commands should be executed
at compile time. Thus, the
compiler should:

- compile a subfunction
- if the compilation specifies
  for compile-time execution,
  execute that and read those
  results.

## Based on the GreyHawk VM

- Add a layer of abstraction
  between the language and the
  VM, to re-use existing work.
