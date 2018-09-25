The Warpspeed VM
================

The Warpspeed VM is a sub-project of Disp. The goal is to become a VM that can be used for
other programming languages.

.. warning::

	The Warpspeed VM will most likely be deprecated in exchange for a compiled binary,
  that provides similar core constructs. See "Why Disp will be a Compiled Language" for
  more information.

What is a Warpspeed Feature vs Disp?
************************************

Warpspeed's goal is to be a VM that enables high-performance of any language
that runs on it. Features inherent to Warpspeed are:

* spawning a worker thread per CPU
    * thread pinning to ensure cache locality
* green threading
* TODO: JIT / AOT compilation

In contrast, Disp focuses on:

* macro support
* homoiconic syntax and data


.. toctree::
   :maxdepth: 2
   :glob:

   *
