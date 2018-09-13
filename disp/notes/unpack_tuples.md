# Unpack Tuples

'tuples' is probably the wrong word, but
the idea is to provide a way to name
variables as they get returned as well.

Something like:

    def return_operation():
        return {left: left, right: right, operand: AND}

    op = return_operation()
    assert op.operand == AND
