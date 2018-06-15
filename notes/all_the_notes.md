### Inferred typing

We can create a system that enabled types to be inferred, via return value.
This assumption can be checked at compile time, and retrieved using a language server.

For example, a chain of functions like:

def add {a: Int, b: Int}:
    return a + b
