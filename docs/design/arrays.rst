Arrays
======

Unlike c and similar to Rust, arrays are a pairing of two values:

* a pointer to the raw array
* a word that is the explicit length of the array

This convenience was chosen primarily to reduce the amount of work
to retrieve the length of said array.