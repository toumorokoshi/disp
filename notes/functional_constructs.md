# Primitive Imperative Constructs

There are only two imperative constructs
built into the VM directly:

1. loop
2. match

That is because these are the root imperative constructs, from which
any construct can be created.

## for loop

    let i = 0
    loop (< i (len my_list))
        let elem = (index my_list i)
        body my_list
        (= i (+ i 1))

## if / else

    match condition {
        true: [(print "true")]
        false: [(print "false")]
    }
