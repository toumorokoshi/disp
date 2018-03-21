# Working with types

Component-based object tend to be a lot better than a
single-inheritance model. If you look at a single inheritance model,
it always falls short when you start to desire the functionality of
another component or type.

A couple use cases are described here:


## Yui's object model

YUI is interesting, because it allows three different methods to compose an object:

* an inheritance model (based off of the typical single inheritance)
* an extension model, where you have an object that then has multiple
  methods folded into it. This somewhat mimics the component-based
  model, except:
  * not all objects are on equal ground. the latest one wins
* a 'plugin' model. Plugins are added on the fly and modify the
  functionality of an instance, not a class (prototype)

The expressed desires of each of these are different. It doesn't seem
like inheritance provides anything useful that the extension model can't handle:

* extensions: I want to compose my class out of multiple components
* plugins: I want to modify the functionality of my object

It would be cool to support this in Greyhawk, while dealing with some of the annoying things about extensions:

* the "last-one-wins" model: this can be confusing for developers, especially if an extension folded in has a method you weren't expecting.

## Component-based inheritance in greyhawk:

you would probably specify your components in greyhawk with something like this:

  class Foo:
    int FooMethod():
      return 1

    int ConflictingMethod():
      return -1

    void OtherConflictingMethod():
      pass


  class Bar:
    int BarMethod():
      return 2

    int ConflictingMethod():
      return 1

    void OtherConflictingMethod():
      pass

  class FooBarObject(Foo, Bar):
    int FooBarMethod():
      return FooMethod() + BarMethod()

    ConflictingMethod = Foo.ConflictingMethod

    void OtherConflictingMethod():
      map(OtherConflictingMethod(this), this.__parents)

so this is interesting because it shows the following:

* if two classes have the same method name, they will raise a compile error. They must be resolved by hand
* there are multiple methods to resolve methods:
  * assign the method to an existing implementation
  * roll your own implementation (including just running all the parent methods like in the above example)

# Questions:

* How can we handle multiple inheritance with classes with the same
  method name but different signatures? It won't be able to adhere to
  both interfaces (without some crazy syntax to let it do so)

# Make things things easy:

* copying an array (if we use slices a la Go this would be easy)
