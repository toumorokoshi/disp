Function Design
===============

For Disp, functions are declared as such:

.. code-block:: 

    fn typecheck [
        {:n resolver :t TypeResolver :d "type resolver to use" :e (new TypeResolver)}
        {:n function :t TypevarFunction}
        {:n args     :t (Vec TypeVar)}
    ] GenericResult 
        let type-var (create_type_var resolver)
        add_constraint resolver (IsLiteral type-var Type::Bool)

arguments are represented by dictionaries. The following symbol keys modify the argument behavior:

* :n is the name of the the argument as referred to by the function body and by callers 
* :t is the type of the parameter (this is optional. if this does not exist it will be inferred)
* :d is the description of the parameter.

The third argument is the return type.

The fourth argument is the body. In the example above it is the body of the idented block (implicit list)

Keyword Arguments / Default Params Work Wonders for Backwards Compatibility
---------------------------------------------------------------------------

Backwards-incompatible changes for functions can result in the need for significant 
refactors and incompatible binary interfaces. A common example of this is Java: as
new arguments are added, the interface changes, and requires recompilations and 
code to be modified, or for the author to support every possible permutation of arguments.

There are two different kinds of interface changes, with common function calls:

=======================
Reordering of Arguments
=======================

If a positional argument is re-ordered, the argument types change, and as a result
every caller must modify their calls.

This can be avoided in languages such as Python, but using the name-value pair on every function call:

.. code-block:: python

   call_functions(argument_1=foo, argument_2=bar,...)


In practice, most library authors can maintain the order of positional arguments.

====================
Adding New Arguments
====================

Oftentimes it is desired to modify function signatures by passing
new arguments. This can happen in the case of new flags.

Languages that support optional arguments can add new arguments with ease,
as you just provide a default value. By not requiring new arguments to
be passed, old callers will work and backwards compatibility is preserved.
