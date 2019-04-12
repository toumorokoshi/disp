Working with Strings and Text
=============================

As a good primer to working with strings in general,
both the Go blog and Rust book have great resources:

* https://blog.golang.org/strings
* https://doc.rust-lang.org/book/ch08-02-strings.html?highlight=string#what-is-a-string

The major takeaway is that common text formats such as UTF-8 are hard to quickly index, unless
you scan the whole string beforehand.

To that end, Disp deals with bytes and text in a similar way.

Bytes
-----

Bytes are effectively an array of bytes. They can be indexed quickly O(1).
Bytes must be converted to some form of textual representation.

Strings
-------

In the common case for developers, it is not necessary to be particular about the specific string encoding. As such, the "String" type is represented by a common format: UTF-8.

Strings are stored in UTF-8 representation for a couple reasons:

* backwards compatible with ascii, without increasing the size
* can represent all characters in known language character sets

Performance of Indexing of Strings
----------------------------------

Due to the variable byte nature of unicode encodings, it's not possible to 
achieve O(1) indexing of strings without scanning the characters.

Some languages achieve O(1) at the cost of incorrectness in some cases, such as Java (`charAt always assumes UTF-16, resulting in incorrect indexes for values that require a larger character set <https://stackoverflow.com/questions/6461402/java-charat-and-deletecharat-performance>`_).

As such, it is not possible to achieve fast indexing. As UTF-8 is a superset of ascii, it is possible to optimize the performance of the String if it is identified as being ascii. (at that point it's just finding a byte offset).

In many cases a string being ascii is sufficient, and can result in a significantly higher performance profile O(1) with an offset lookup vs O(n) with a scan:

* developer logging
* API error messages could be reduced to english

All of this implies that the "string" type in disp should be a layer of abstraction that ensures performance in many common cases, although there should still be space for a more custom data type. (potentially allow compile-time reading of literals.

Individual characters in a string are referred to as Runes
----------------------------------------------------------

It is common nomenclature to call a single byte in an ascii string a "char"
or character. As UTF-8 strings can be multiple bytes, we will use the word "Rune" to describe a single UTF-8 character.

No cost to convert from Bytes to a String
-----------------------------------------

It's often value to parse raw bytes into text, as the bytes being read in
are guaranteed to be UTF-8.

In that case, it's possible to do a simple type coercion at no runtime cost.

