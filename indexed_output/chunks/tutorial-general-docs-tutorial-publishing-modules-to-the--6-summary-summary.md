---
doc_id: tutorial/general/docs-tutorial-publishing-modules-to-the-
chunk_id: tutorial/general/docs-tutorial-publishing-modules-to-the-#6-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 140
summary: with the lower-cased form of YOUR GitHub username. For example:
---

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
