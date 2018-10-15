Homoiconic Syntax
=================

A language is considered homoiconic if the language itself can be represented with primitive data structures.

Disp satisfies that, as the language itself can be represented with only the following data structures:

* lists
* maps
* TBD: sets

Easy to Transform
*****************

By using simple data structures, it reduces the complexity of transforming the ast in various ways. For example, one can easily add a match condition by reading in a map and adding a key-value branch:

.. code-block:: disp

  // add false to the syntax tree.
  macro! add-false-handler [map-foo]
    add map-foo false
