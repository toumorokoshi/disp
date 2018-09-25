Disp Syntax
===========

Disp's syntax is homoiconic: it is authored with the same syntax that one
would use to represent data. The data types that can be represented natively are:

* integers: 0..6+
* strings: \".+\"
* lists: [token*]
* expressions: (token*)
* maps: {key: value, key2: value2,+}

Expressions
***********

Expressions are represented in the language as:

.. code-block:: lisp

  (function-name arg1 arg2 ...)


This will be executed by the compiler, rather than represented as data.
Sometimes, it's valuable to defining an expression object without
