# Design Decisions for Disp

## Homoiconic

There's a lot of power in being able to modify the code itself with the same functions that one users with regular code:

* strengthens your ability to work with macros
* the same power of the language can be applied to transformations of the language itself

## Favor pure functions over stateful ones

Pure functions (ones that have no side effects) are very easy to debug, and the only interface is in the return value, vs the passed value.

Disp could have some syntax implying a pure function vs an impure one, like:

    defun foo [mut] {a: Int} Int

To help tag that this function does mutate and has side effects.

## Non-blocking I/O is the default

Non-blocking I/O is very easy to scale in a single thread with less memory overhead. It's also
very easy to adapt non-blocking I/O to a thread based model, so we should support that.

## Primitive Imperative Constructs

There are only two imperative constructs
built into the VM directly:

1. loop
2. match

That is because these are the root imperative constructs, from which
any construct can be created.

### for loop

    let i = 0
    loop (< i (len my_list))
        let elem = (index my_list i)
        body my_list
        (= i (+ i 1))

### if / else

    match condition {
        true: [(print "true")]
        false: [(print "false")]
    }

# To Solve

## Simple Concurrency

How to ensure simple concurrency? reduce the mental overhead of locks / etc?

options:

* actors

## Extending the Language

It would be really cool if the language didn't support constructs like type checking
in the core, but that support could be added on. For example, we could provide a simple
way to extend the type function definitions, or variable definitions, to enable such
additional attributes to be consumed via a compiler plugin.

Example is with type checking


    def multiply [l, r]
        * l r

With type checking:

    def multiply [l: int, r: int]
        * l r


But that won't include other things, like constraints:

    def multiple [l: int, > 0, r: int, > 0]
    def multiple [l: {type: int, constraint: > 0}


Or the http argument that it should represent:

    def multiple [
      l: {type: int, argument: query},
      r: {type: int, argument: query}
    ]

# Cool things other language are doing

## A 'remove' keyword in a loop.

Jonathan Blow's suggest a new 'remove' keyword in a loop for his own language:


    for p : pointers {
        if (p.value % 2 == 0) {
            remove p
        }
    }

Basically, a keyword that handles removal of an element from an
array. This is desired because removing an element from an array is tricky.

# Declarative programming

* when querying data, declarative programming works really well:

SELECT x FROM y WHERE {{query}}

It'd be nice to add that into greyhawk somehow.


# Syntax

##  Braces

There's 5 types of braces:

{}
[]
/\
()
<>

There's a lot of uses for these:

* denote a set: {'foo', 'bar'}
* denote a list: [1, 2, 3]
* denote an array: [1, 2, 3]
* tuple: (false, ["an error"])
* type union: <Int, None>
* generics: List<[Char]>
* a map: {foo: 'bar', bar: 'baz'}

### Current Implementation

* union types: {Int, None}
* union w/ None: Int?
* tuple: (Int, None)
* Set: {1, 2, 3}
* List: [1, 2, 3]
* Array: Array[1, 2, 3]
* generics: List<Char>, List<{Char, Int}>
* map: {"foo": "value", "bar": "baz"}

### Ideas

#### Lists can be default, Array could be specified:

    a := [1, 2, 3]
    a := List<Char>[1, 2, 3]
    a := Array<Int>[1, 2, 3]
    a := LinkedList<Char>[1, 2, 3]

#### Unions will be very important, so it should be easy to type.

    [Int, None] a
    <Int, None> runStuff()
    {Int, None} runStuff()
    (Int, None)
    (Bool, {[Char], None}) executeFunctionWithErrors()
    (Bool, Char?) executeFunctionWithErrors()

# Loading Modules

I've had a lot of frustration with trying to get Python to do
isolation properly. There's a few caveats that have made things very difficult:

## allowing the stdlib to be placed in the same directory as installed packages.

For example, allowing someone to install requests into /usr/lib/pythonXX/. There's no way
to distinguish between a package that was intalled as an stdlib object, or a package that was
installed by a user.

Of course, GreyHawk may not have that problem, because it's aim is to have the stdlib itself be EXTREMELY small, and
to, on installation (or one could package), install the stdlib at default versions.

## Allowing the versions of stdlib to drift.

This is a bit of a dilemma. The reason why a lot of people never made
the switch to Python3 mainly had to do with the fact that packages
have moved around in a backwards-incompatible way. This effectively
forces someone to choose between python 3 and python 2, or have the
understanding to create a dual-compatible framework.

Greyhawk could avoid this problem altogether by packaging the basic language with a very small number of dependencies, and
allowing people to pick and choose, upgrading as they wish.

However, this would really disencourage shared versions across
multiple projects. It's basically downright impossible: people could
use whatever versions of whatever dependencies to build their package,
and that would conflict all the time as soon as someone made a choice.

So, to combat that, Greyhawk need really, really good isolation. It should by easy too:

* have a service install things relative to a particular directory


## The dependency managements system

* should be written in Greyhawk
* should allow alternatives DM systems
* will install packages locally (Greyhawk as a language should
  disallow global packages). It should have a way to install CLIs
  though.


## Casing: Types should be uppercase, type variables should be lowercase

* functions with the Type in question should always return that type.