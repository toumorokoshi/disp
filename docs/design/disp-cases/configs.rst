Using Disp for Configuration
============================

At Zillow in 2018, we had a process to pass environment-specific configuration to an application that looked like:

1. grab configuration values for a specific environment
2. render template file with that configuration (typically jinja + yaml)
3. load configuration during application startup

It typically looked something like this:

.. code-block:: yaml

  config:
    dns: {{ my_application.dns }}
    port: {{ my_application.port }}
    database:
      connection_string: {{ my_application.db.connection_string }}

This works well, but at scale it runs into a couple problems. One is that common configuration ends up being littered across multiple applications, so you end up with boilerplate configuration everywhered.

The other is that people like to get fancy. Jinja as a language supports a broad set of conditional logic, so you can do stuff like completely change the rendered file based on a config value::

  # fancy set, if / else conditions at the top
  {% set svc = my_application %}
  config:
    dns: {{ svc.dns }}
    port: {{ svc.port }}
    database:
      connection_string: {{ my_application.db.connection_string }}
    # or a common configuration, such as:
    application_metrics:
      consumer: {{ application_metrics.consumer.host }}
      port: {{ application_metrics.consumer.port }}
      default_namespace: my_application


When you have a a lot of boilerplate, at some point a backwards incompatible change comes along, and you need to go change the configuration for everybody. This is a relatively easy task for vanilla yaml: you just need to:

1. iterate through all repos containing this configuration
2. load yaml file
3. rewrite value
3. write yaml file

This loses comments since they aren't part of the data itself, but the application still works as expected.

Unfortunately, you cannot use the Yaml parser to load yaml templatized with Jinja syntax: the brackets {} are parsed by Yaml as an object, as Yaml is a superset of Json. As a result, a simple task like adding a specific value in a specific location in a Yaml hierarchy now becomes an extremely difficult task. In order to do this properly, you would need to:

1. Load the Jinja template into it's data structure representation.
2. Replace variables with placeholders.
3. Hope that the result can be parsed with yaml.
4. If 3 is true, load the values with yaml, and write your value in question.
5. Replace placeholders back with real values.
6. Merge the results with the Jinja data structure you have.

Unfortunately very rarely were all of those aligned, so it becomes effectively impossible to do programmatic updates. Anything that can't be automated at scale means it takes a significant amount of manual effort to accomplish.

How Disp Would Help
*******************

The best solution to the above issue is probably decoupling your common components enough that you can avoid this issue (such as having configs live with the library that uses them, and have them source the config directly somehow), but that can often be tricky, and difficult to ensure that no one makes a mistake and causes boilerplate to happen.

The issue with the situation above is that multiple languages are at play. Anytime you mix syntax of two different systems, it is very difficult to write a parser that somehow supports both.

Decomposing the purpose of the two languages above, they are:

1. allow conditional logic and variables (Jinja)
2. structure data so that it can be read by the application (Yaml)

Disp's syntax tree is nothing but lists, maps, and other basic data types: the same ones that would be present in a language like Yaml. When combined with a Disp interpreter, you can execute the code and return the data you're looking for:

.. code-block:: lisp

  let val (json (read))
  return {
    config: {
      dns: (get (get val "my_application") "dns")
      port: (get (get val "my_application") "dns")
    },
    database: {
      connection_string: (get (get (get val "my_application") "db") "connection_string")
      port: (get (get val "my_application") "dns")
    }
  }
