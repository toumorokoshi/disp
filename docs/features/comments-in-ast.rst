Comments in the AST
===================

The AST also includes comments as a first-class type.
This is important, as it allows for:

* ast transformations to preserve comments, including comment location
* enables extract of comment information from the AST, such as for building of documentation
