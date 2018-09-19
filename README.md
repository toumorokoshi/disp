# Disp

Disp is a programming language designed to scale to codebases of all sizes.

## Attributes

Here's some of the attributes to give an idea of what Disp is about:

* compiles to bytecode, then interpreted
* homoiconic
* compile-time macros
* spawns one worker process per cpu
* each worker runs an event loop (event loops can share data)

## The Goals of Disp

When someone starts a software project, almost any language will work, and many start with their personal preference. As the project increases in size and complexity, the language choice starts to matter a lot. The language eventually falls short in some way that results in developers making a painful choice, sometimes even moving away from the language they chose in the first place.

There are no limit to possible shortcomings, but to name a few:

* performance: my language is not fast enough
* difficult to make interface changes: I made a backwards incompatible change, and it will take significant effort to propagate this change across the organization.

The primary goal of Disp is to be a language that doesn't require someone to make that painful decision: it should be a language that scales to a codebase of any size, and enable tooling that will help it scale.

This is all very vague, so here's some specific points that will be elaborated on in the future.

### Performance

Disp enables high performance in the following ways:

* large concurrency via green threading (aka fibers)
* cache affinity by enabling fibers sharing data to run on the same cpu
* data locality to maximize cache affinity
* metaprogramming via macros, to reduce the overhead of
  runtime compilation of constant values.
* (PLANNED FEATURE): some compilation, either AOT or JIT.
* (PLANNED FEATURE): compile-time selection of optimal data structure based on usage patterns.

### Easy to Verify and Augment Code Programmatically

A codebase that scales to hundreds of repositories and millions of lines of code results in an increased amount of effort to propagate changes globally, such as interface changes or eliminating inefficient patterns. Performing these changes by hands does not scale, requiring significant amounts of human effort to propagate each change.

It is impossible to eliminate these types of changes, but they can be minimized. In addition, when these changes need to occur, they should be able to be performed programmatically.

Disp enables this by:

* homoiconic syntax: code is written with the same data structures that the developer uses day to day, reducing the barrier to author automated syntax changes.
* homoiconic syntax: the code can be manipulated programmatically, and simply.
* performance: by ensuring high performance, it eliminates the need for a secondary language in many cases, reducing the cost for tooling for multiple languages, or a multi-language boundary.


### Strong Conventions

Large codebases can often suffer from large debates around style, or the approach to specific problems. By codifying these patterns as conventions in the language itself, it eliminates the need for discussions and mass migrations as the conventions change.
