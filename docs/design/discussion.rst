Design Considerations
*********************

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


Developer Productivity
****************

One is developer productivity, ensuring logic is as intended. Developers should be able to efficiently write code and tackle common challenges around coding, including:

* writing new code
* understanding legacy code
* resolving bugs with existing code

Performance
***********

With developer productivity comes a common problem that affects successful software programs: performance. It's often said that performance is often not a primary driver of language choice, and premature optimization is the root of all evil.

However, the optimization process can be significantly more expensive or cheap depending on how the language itself is designed. Examples of language that are difficult to optimize include Python, which require developers to author non-python code to resolve (such as Cython for a Python subset, or C code).

Hard-To-Optimize Language (Python):

* requires coding outside the language to optimize

Easy-To-Optimize Language (Java):

* can get really good performance (1-2x C) before needing to move out of the language
* sticking to the same language enables:
  * easier developer contribution
  * simpler build tooling

Compare this to languages which often do not require significant optimization outside the language, such as Java: the VM itself is efficient enough for a wide range of purposes, and thus enables much better developer contribution
