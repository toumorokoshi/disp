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
