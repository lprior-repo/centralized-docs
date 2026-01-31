---
doc_id: ops/general/docs-howto-about-commented-cue-guides
chunk_id: ops/general/docs-howto-about-commented-cue-guides#2-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary: that cue has caught the deliberate mistakes in our example data. Here’s an example:
---

that cue has caught the deliberate mistakes in our example data.

Here’s an example:

Copied!
example.cue

Copy code
Copied!

package example

// CUE comments start with "//"
// and run to the end of the line

// f1 is a regular field which must be a string
f1: string

// f2 is a required field which must be an
// integer over 10
f2!: int & >10

Copied!
data.yml

Copy code
Copied!

# f1 is actually an integer
f1: 123

# f2 is actually a string
f2: "some string value"

TERMINAL

Copy code
Copied!

$ cue vet -c .:example data.yml
