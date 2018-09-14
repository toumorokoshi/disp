* modify native functions to include it's return type (helps with native
  functions such as Int and read-line
* use result to handle error values returned from the vm function (e.g. parseint failing)
* introduce sets into disp.
* functions can be created that operate on specific interfaces in specific ways, but can be overriden (polymorphism)
* if / else is a little hacky in that the else block is omitted if it doesn't exist,
  which can produce erroneous values. That should be fixed.
