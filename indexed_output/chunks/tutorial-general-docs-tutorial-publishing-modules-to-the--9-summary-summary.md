---
doc_id: tutorial/general/docs-tutorial-publishing-modules-to-the-
chunk_id: tutorial/general/docs-tutorial-publishing-modules-to-the-#9-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 137
summary: Ensure the module. cue file is tidy:
---


5

Ensure the module.cue file is tidy:

TERMINAL

Copy code
Copied!

$ cue mod tidy

6
If you haven’t already done so,
create a repository [https://github.com/new?org=]
called frostyconfig under your personal username at GitHub.
It doesn’t matter if the repository is public or private.
7

Create a git commit:

TERMINAL

Copy code
Copied!

$ git add -A
$ git commit -q -m 'Initial commit'

Earlier, you initialized this module with --source=git, which told the cue
command that it should publish only those files that git knows about. The git
