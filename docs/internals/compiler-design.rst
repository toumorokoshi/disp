Compiler Design
===============

The compiler is split up into multiple phases, to ensure proper abstractions and allow processing of an individual phase.

The phases are:

Tokenization and Parsing
------------------------

* input: string
* output: tokens

This phase reads in the raw text and converts it into 
tokens that can represent the data in a more meaningful fashion.

Function Parser 
----------------

* input: tokens
* output: function and macros with raw expressions

Reading through the parsed data structure, and creating function representations that contain the body and it's expressions

Macro Expansion
---------------

* input: function signatures with raw expressions
* output: function signatures with raw expressions

As part of loading functions, some statements are consumed as Macros. The expressions bodies are now parsed, with any statement matching a macro expanding then and there.

The output looks very similar but omits any macro calls, and the results are the output of the macros.

Type Checking
-------------

* input: function signatures with raw expressions
* output: function signatures with expressions and types of the resulting expression

This phase reads in raw expressions, and attaches the types to them. As it is possible to encounter things such as a recursive expression, type variables are used in this step and use unification to resolve them.

Function Building
-----------------

* input: function signatures with expressions and types of the resulting expression
* output: functions with LLVM builder proxies

This phase handles deciding what the raw LLVM expressions will ultimately be.

The LLVM cannot be built directly, one cannot do things such as reference functions that do not exist yet. This make it difficult to, for example, declare functions in a more arbitrary sequence.

LLVM Builder
------------

* input: functions with LLVM builder proxies
* output: LLVM bytecode

This handles the construction of the LLVM code itself. 

LLVM Compilation

* input: LLVM bytecode
* output: machine code

This outputs machine code, at which point the code may be executed.