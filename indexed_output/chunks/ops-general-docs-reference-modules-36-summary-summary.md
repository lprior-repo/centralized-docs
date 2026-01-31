---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#36-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 136
summary: to file path and size constraints [/docs/reference/modules/#zip-path-size-constraints]. FILE PATH AND SIZE CONSTRAINTS
---

to file path and size constraints [/docs/reference/modules/#zip-path-size-constraints].

FILE PATH AND SIZE CONSTRAINTS

There are a number of restrictions on the content of module zip files. These
constraints ensure that zip files can be extracted safely and consistently on
a wide range of platforms.

 * A module zip file may be at most 500 MiB in size. The total uncompressed size
   of its files is also limited to 500 MiB. module.cue files are limited to 16 MiB.
   LICENSE files are also limited to 16 MiB. These limits exist to mitigate
