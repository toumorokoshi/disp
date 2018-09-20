Design Discussion
*****************

This section is mainly here for brainstorming.

Indentation is important
========================

In clojure, it's very easy to leave additional statements that will be executed on a line above, looking like:


.. code-block:: clojure

  (dotimes [i 10] (readline)
    (readline)
  )

In this case, we will read from stdin twice per loop, as the first
statement after the dotimes input is on the same line as the declaration of the dotimes.

It's a lot easier to look at the shape and understand the a block that will be executed is on the next line, and see that block as whole:

.. code-block:: python

  for i in 10:
    sys.stdin.readline()
    sys.stdin.readline()

It's very clear here.

Deep Imports Are Helpful
========================

Rust provides import syntax that allows ones to import specific
functions from modules, like:

.. code-block:: Rust

  use module::{
    namespace::value,
    namespace2::{value1, value2},
    namespace3
  }

This allows for:

* imports grouped by root location: helpful to see if an import is already added
* reduces


Use explicit "return" keyword
=============================

Rust allows two ways to return a value: either as the last execution in a statement, or with a return statement. The lack of a return statement makes it difficult to search for where termination would occur.

Conversely, it does make it difficult to support single expressions, since that would be easily written as a single expression and it would be clear that it is the return value. Maybe I'm not sure where I stand on this yet.
