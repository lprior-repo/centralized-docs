---
doc_id: tutorial/general/docs-tutorial-publishing-modules-to-the-
chunk_id: tutorial/general/docs-tutorial-publishing-modules-to-the-#1-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 521
summary:    or a Windows terminal such as PowerShell, cmd, or.    WSL [https://learn
---

   or a Windows terminal such as PowerShell, cmd, or
   WSL [https://learn.microsoft.com/en-us/windows/wsl/install]
   to run commands.
 * Some awareness of CUE schemata –
   the language tour’s pages on
   Constraints [/docs/tour/basics/constraints/] and
   Definitions [/docs/tour/basics/definitions/] are a good refresher

This tutorial is written using the following version of cue:

TERMINAL

Copy code
Copied!

$ cue version
cue version v0.15.3
...

CREATE THE MODULE FOR THE SCHEMA CODE

In this tutorial we will focus on an imaginary application called FrostyApp,
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
cueckoo,
with the lower-cased form of YOUR GitHub username.
For example:
if your GitHub username is _TomHanks
then you would replace cueckoo with _tomhanks.

You need to make this replacement everywhere you see
the username cueckoo in this tutorial.

2

Initialize the directory as a git repository and a CUE module:

TERMINAL

Copy code
Copied!

$ git init -q

# Replace "cueckoo" with *your* GitHub username, lower-cased.
$ cue mod init --source=git github.com/cueckoo/frostyconfig@v0

The --source=git flag tells cue to use the same file-inclusion rules as
git, when publishing this module.

The GitHub user cueckoo controls all the repositories under
github.com/cueckoo/, so they can publish modules to the Central
Registry inside that namespace. The same is true for your GitHub username.

3

Create the configuration schema:

Copied!
frostyconfig/config.cue

Copy code
Copied!

package frostyconfig

// #Config defines the schema for the FrostyApp configuration.
