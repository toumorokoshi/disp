# codename: DISP

DISP is an experimental programming language, with the goal of
providing metaprogramming without significant sacrifices to
performance.

## Goals

DISP has many planned features, but the overall philosophy driving it
can be summarized

### no "two language" problem

Many flexible languages require one to learn two languages to achieve
high-performance in applications:

1. the primary language in question
2. a secondary, lower level language (such as C) to help areas that
   need to be optimized.

Although disp will allow FFI for languages that a natively compiled
language, the goal is to only require it in the most critical
performance requirements.

## Features

the following features define disp:

* intepreted (runs on greyhawk-vm)
* statically typed
* any expression can be evaluated compile time for optimization
* homoiconic
