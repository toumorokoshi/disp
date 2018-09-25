Type Inference
==============

Similar to ML and Haskell, Disp supports type checking using type inference, enabling
static type validation without the need to specify types signatures in most situations.

For example, you can define a function without types:

.. code-block:: lisp

  let p (fn [arg] [(print arg)])
  p "foo"
  p 10

Note that there are two different types passed into "p": a string, and an integer.
In this situation, the compiler knows that the print function supports both strings
and integers, and will generate separate functions that take their respective types.

On compilation time, disp stores this type of information on all functions, so this
relationship is preserved regardless of the functions that are used.

This enables behavior similar to `duck-typing <https://en.wikipedia.org/wiki/Duck_typing>`_,
But retaining the ability to type check on compile time. For example, if a function called
in the body of a parent function does not support the type in question, the compiler will
raise an error:

.. code-block:: lisp

  mut add (fn [l r] [(+ l r)])
  // this will work
  println (add 1 10)
  // this will raise a type check error on compilation,
  // since the "+" function does not support strings
  println (add "foo" 10)
