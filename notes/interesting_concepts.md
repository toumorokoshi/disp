## A 'remove' keyword in a loop.

Jonathan Blow's suggest a new 'remove' keyword in a loop for his own language:


    for p : pointers {
        if (p.value % 2 == 0) {
            remove p
        }
    }

Basically, a keyword that handles removal of an element from an
array. This is desired because removing an element from an array is tricky.
