---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#3-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: for instance:.  * a single language for specifying data, schema, validation
---

for instance:

 * a single language for specifying data, schema, validation
   and policy constraints,
 * meta reasoning, such as determining whether
   a new schema version is backwards compatible,
 * automated rewriting, such as is done by cue trim,
 * creating multi-source constraint pipelines, retaining documentation
   across normalization,

and so on.

THE VALUE LATTICE

Every value in CUE, including what would in most programming languages
be considered types, is partially ordered in a single hierarchy
