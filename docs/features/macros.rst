Macros
======

As with other lisps, Disp supports macros. Macros are evaluated compile time and resolve to an ast that is then evaluated by the compiler.

For example, one could create something equivalent to an 'unless' keyword by writing:

.. code-block:: lisp

	macro! unless [conditional] body
		' while (not conditional) body

This will expand:

.. code-block:: lisp

	unless (eq i x)
		print i
		mut i (+ i x)


into:


.. code-block:: lisp

	while (not (eq i x))
		print i
		mut i (+ i x)

Design Decisions
****************

Why are macro invocations not prefixed with a "!"?
--------------------------------------------------

A compiled macro can be argued as a bang expression, since it
invokes behavior on compile time (transforming the arguments into
a different syntax tree).

There rationale for making it look like a regular function is:

* macros should be seen as equivalent to builtins, which are not
	prefixed with a bang.
* bang symbols imply that the full statement will be executed on compile
	time. In the case of macros, the macro expansion will be invoked, but the
	body of the code will not.
* this also enables execution of a macro, and it's contents, during compile time.
  so both of these situations are possible:

.. code-block:: lisp

	# runtime evaluation
	let x (unless true (print bar))
	# compile time evaluation
	let y (unless! true (print bar))
