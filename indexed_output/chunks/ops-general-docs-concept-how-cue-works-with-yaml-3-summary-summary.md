---
doc_id: ops/general/docs-concept-how-cue-works-with-yaml
chunk_id: ops/general/docs-concept-how-cue-works-with-yaml#3-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 136
summary: $ cue export --out yaml data. In addition to YAML, cue can read and write
---

}


Copy code
Copied!

g: "seven"
h: 4.4 * 2

TERMINAL

Copy code
Copied!

$ cue export --out yaml data.yml data.json data.cue
a: 1
b: "2"
c: three
d: 4.4
e: 5
"f": "6"
g: seven
h: 8.8

In addition to YAML, cue can read and write
a range of other formats [/docs/integration/].

VALIDATING YAML FILES AGAINST A SCHEMA

CUE is often used to make systems safer without having to teach the underlying
system components about CUE. Because the cue tool can validate YAML files
using CUE’s powerful and compact constraint syntax, it’s easy to add
