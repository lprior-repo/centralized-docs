---
doc_id: tutorial/general/docs-tutorial-publishing-modules-to-the-
chunk_id: tutorial/general/docs-tutorial-publishing-modules-to-the-#5-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 135
summary: which consumes its configuration in YAML format. You will define the configuration in CUE and use a CUE schema to validate it
---

which consumes its configuration in YAML format.
You will define the configuration in CUE and use a CUE schema to validate it.
We would like to be able to share the schema between several consumers,
so we will publish it to the Central Registry.

1

Create a directory to hold the schema code:

TERMINAL

Copy code
Copied!

$ mkdir frostyconfig
$ cd frostyconfig

You need to adapt the command shown in the next step.

Don’t simply paste the command into your terminal and run it.

Before running the command, replace the example username,
