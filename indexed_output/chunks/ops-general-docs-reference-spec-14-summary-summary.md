---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#14-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: Unless noted otherwise, they can also be used as identifiers to refer to. the same name
---

Unless noted otherwise, they can also be used as identifiers to refer to
the same name.


VALUES

The following keywords are values.


Copy code
Copied!

null         true         false

These can never be used to refer to a field of the same name.
This restriction is to ensure compatibility with JSON configuration files.


PREAMBLE

The following keywords are used at the preamble of a CUE file.
After the preamble, they may be used as identifiers to refer to namesake fields.


Copy code
Copied!

package      import
