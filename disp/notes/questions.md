## Should structs be initialized?


## Scope statements?
http://dlang.org/statement.html#scope-guard-statement

## Compile time execution?

this will probably happen by loading the module. more thought should
probably be put in on how to make this clear.

## Static assert

assertions that occur during compile time. It adds additional ways for the
compiler to fail.

## automatic component passing
(inspired by JAI)

one of the tricky parts of composition is the requirement to access
subattributes:

    type Person:
        int age
        char[] name

    type Employee:
        Person person
        int employeeId

    int getAge(Employee e):
        return e.person.age

    int getAgeFromPerson(Person p):
        return p.age


This is brittle and verbose. Instead, one could just automatically pass in subattributes as necessary:

    int getAgeFromPerson(Person p):
        return p.age

    // this automatically accesses the subattribute 'person' and uses that value.
    age := getAgeFromPerson(e)
