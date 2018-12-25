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


## Compiler V2

In order to properly support type inference and more complex operations during
resolution, the compiler needs to be more complex.

### Phase 1: Function Scanning

To enable function overloading, one requires the full set of overloads to test
against.

* build a map of function names to method signatures.

### Phase 2: Type Inference

  for function in all_functions:
    # collect type variables (do so recursively as well)
    build_instructions_with_type_variables = resolve_types(function)
    # this should be happening real-time, as we have a real-time resolver
    # now.
    resolve_variables(compiler)
    # build actual llvm builder instructions
    build_function()
