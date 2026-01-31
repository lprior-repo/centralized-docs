---
doc_id: ops/general/docs-introduction-installation
chunk_id: ops/general/docs-introduction-installation#5-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 140
summary: installed and available. For example, to fetch the latest version:
---

installed and available.

For example, to fetch the latest version:

TERMINAL

Copy code
Copied!

$ go install cuelang.org/go/cmd/cue@latest
...

This page [https://pkg.go.dev/cuelang.org/go?tab=versions]
lists the installable releases and pre-releases that you can specify instead of
latest.


DEVELOPMENT VERSION

You can install the development version of cue from source by specifying master:

TERMINAL

Copy code
Copied!

$ go install cuelang.org/go/cmd/cue@master
...

The capabilities of the development version change frequently because it contains the
