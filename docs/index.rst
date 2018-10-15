.. disp documentation master file, created by
   sphinx-quickstart on Wed Sep  5 22:17:24 2018.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

Disp
====

Disp is a programming language designed with the goals of:

* high performance by default: choices are built in that ensure mechanical sympathy.
* managing large codebases easily: reducing cost for refactors, managing code across multiple code repositories.

The specific features of Disp include:

* homoiconic syntax: code is represented using standard data structures
* macro support: function that can do compile-time syntax evaluation
* compile-time execution: functions can execute on runtime, or compile time

.. warning::

  Disp is not ready for use in any production project! Literally anything about the language can change until 1.0


You can see a code example here (solving the qualifier problem a for Google Code Jam 2008):

.. literalinclude:: ../examples/gcj_2008.ds
  :language: lisp

To get a better understanding of where Disp shines, it's recommended to read up on the features, and some disp-cases (use cases that inspired disp's design).


Features
********

.. toctree::
  :maxdepth: 1
  :glob:

  features/*



Table of Contents
*****************

.. toctree::
   :maxdepth: 2
   :glob:

   design/discussion
   design/performance
   design/productivity
   design/disp-cases
   one-point-zero
   design/*
   warpspeed/index


Disp-Cases
**********

.. toctree::
  :maxdepth: 1
  :glob:

  design/disp-cases/*


Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
