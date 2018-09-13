# Braces

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

## Current Implementation

* union types: {Int, None}
* union w/ None: Int?
* tuple: (Int, None)
* Set: {1, 2, 3}
* List: [1, 2, 3]
* Array: Array[1, 2, 3]
* generics: List<Char>, List<{Char, Int}>
* map: {"foo": "value", "bar": "baz"}

## Ideas

### Lists can be default, Array could be specified:

    a := [1, 2, 3]
    a := List<Char>[1, 2, 3]
    a := Array<Int>[1, 2, 3]
    a := LinkedList<Char>[1, 2, 3]

### Unions will be very important, so it should be easy to type.

    [Int, None] a
    <Int, None> runStuff()
    {Int, None} runStuff()
    (Int, None)
    (Bool, {[Char], None}) executeFunctionWithErrors()
    (Bool, Char?) executeFunctionWithErrors()
