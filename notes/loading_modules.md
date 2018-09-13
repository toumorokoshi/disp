# Loading Modules

I've had a lot of frustration with trying to get Python to do
isolation properly. There's a few caveats that have made things very difficult:

## allowing the stdlib to be placed in the same directory as installed packages.

For example, allowing someone to install requests into /usr/lib/pythonXX/. There's no way
to distinguish between a package that was intalled as an stdlib object, or a package that was
installed by a user.

Of course, GreyHawk may not have that problem, because it's aim is to have the stdlib itself be EXTREMELY small, and
to, on installation (or one could package), install the stdlib at default versions.

## Allowing the versions of stdlib to drift.

This is a bit of a dilemma. The reason why a lot of people never made
the switch to Python3 mainly had to do with the fact that packages
have moved around in a backwards-incompatible way. This effectively
forces someone to choose between python 3 and python 2, or have the
understanding to create a dual-compatible framework.

Greyhawk could avoid this problem altogether by packaging the basic language with a very small number of dependencies, and
allowing people to pick and choose, upgrading as they wish.

However, this would really disencourage shared versions across
multiple projects. It's basically downright impossible: people could
use whatever versions of whatever dependencies to build their package,
and that would conflict all the time as soon as someone made a choice.

So, to combat that, Greyhawk need really, really good isolation. It should by easy too:

* have a service install things relative to a particular directory


## The dependency managements system

* should be written in Greyhawk
* should allow alternatives DM systems
* will install packages locally (Greyhawk as a language should
  disallow global packages). It should have a way to install CLIs
  though.
