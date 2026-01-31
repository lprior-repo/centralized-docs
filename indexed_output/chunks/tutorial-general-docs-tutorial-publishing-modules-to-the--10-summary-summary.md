---
doc_id: tutorial/general/docs-tutorial-publishing-modules-to-the-
chunk_id: tutorial/general/docs-tutorial-publishing-modules-to-the-#10-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: commit you just created leaves the directory in a “clean” state, which is. necessary for cue to know exactly which files to include in the published
---

commit you just created leaves the directory in a “clean” state, which is
necessary for cue to know exactly which files to include in the published
module.

8

Publish the first version of this module:

TERMINAL

Copy code
Copied!

$ cue mod publish v0.0.1
...

This command should mention your GitHub username,
and should publish the module successfully.

If the command fails with an error message that mentions your GitHub username
then you probably haven’t created the frostyconfig repository under your GitHub username.
