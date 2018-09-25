Macros
======

As with other lisps, Disp supports macros. Macros are evaluated compile time and resolve to an ast that is then evaluated by the compiler.

For example, one could create something equivalent to an 'unless' keyword by writing:

.. code-block:: lisp

	!macro unless [conditional] body
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
