---
doc_id: ops/general/docs-tour-packages
chunk_id: ops/general/docs-tour-packages#2-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 142
summary: order does not matter [/docs/tour/basics/order-irrelevance/]. package config
---

order does not matter [/docs/tour/basics/order-irrelevance/].

Copied!
policy.cue

Copy code
Copied!

package config

foo:  bar/2 - 1
bar!: int

Copied!
data.cue

Copy code
Copied!

package config

bar: 200

TERMINAL

Copy code
Copied!

$ cue export # No filenames mentioned
{
    "foo": 99,
    "bar": 200
}

The cue tool processes lists of CUE files and package paths.
Because working with a single package split across multiple files in the
current directory is such a common situation,
cue processes that single package if it isn’t told to look at anything else.
