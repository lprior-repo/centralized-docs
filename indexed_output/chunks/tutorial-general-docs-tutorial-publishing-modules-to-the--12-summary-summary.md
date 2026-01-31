---
doc_id: tutorial/general/docs-tutorial-publishing-modules-to-the-
chunk_id: tutorial/general/docs-tutorial-publishing-modules-to-the-#12-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: changing cueckoo to your GitHub username, lower-cased:. $ git init -q
---

changing cueckoo to your GitHub username, lower-cased:

TERMINAL

Copy code
Copied!

$ mkdir ../frostyapp
$ cd ../frostyapp
$ git init -q
$ cue mod init --source=git github.com/cueckoo/frostyapp@v0

10

Create the code for the new module:

Copied!
frostyapp/config.cue

Copy code
Copied!

 1
 2
 3
 4
 5
 6
 7
 8
 9
10


package frostyapp

// Adapt this line to your GitHub username, lower-cased.
import "github.com/cueckoo/frostyconfig@v0"

config: frostyconfig.#Config & {
	appName: "alpha"
	port:    80
	features: logging: true
