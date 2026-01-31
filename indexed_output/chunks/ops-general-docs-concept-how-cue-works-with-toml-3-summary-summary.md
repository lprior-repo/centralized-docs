---
doc_id: ops/general/docs-concept-how-cue-works-with-toml
chunk_id: ops/general/docs-concept-how-cue-works-with-toml#3-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 137
summary: b: d: i: g + b. $ cue export --out toml a
---


b: _
g: _

h: "six"
b: d: i: g + b.d.e

TERMINAL

Copy code
Copied!

$ cue export --out toml a.toml b.json c.cue
a = '1'
f = '4'
g = 5.5
h = 'six'

[b]
c = 2.2

[b.d]
e = 3
i = 8.5

The cue command can read and write
a range of other formats [/docs/integration/]
as well as TOML.

VALIDATING TOML FILES AGAINST A SCHEMA

CUE is often used to make systems safer without having to teach the underlying
system components about CUE. Because the cue tool can validate TOML files
using CUE’s powerful and compact constraint syntax, it’s easy to add
