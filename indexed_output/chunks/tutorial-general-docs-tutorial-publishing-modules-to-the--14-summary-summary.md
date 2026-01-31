---
doc_id: tutorial/general/docs-tutorial-publishing-modules-to-the-
chunk_id: tutorial/general/docs-tutorial-publishing-modules-to-the-#14-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 139
summary: Export the configuration as YAML:. $ cue export --out yaml
---


12

Export the configuration as YAML:

TERMINAL

Copy code
Copied!

$ cue export --out yaml
config:
  appName: alpha
  port: 80
  features:
    logging: true

We can use this new module code just like any other CUE code.

CONGRATULATIONS!

That’s it! You have just created a module and published it to the Central
Registry, and then used the newly published module to check a concrete
configuration held in a different module.

RELATED CONTENT

 * Tutorial: Working with modules and the Central Registry [/docs/tutorial/working-with-the-central-registry/]
