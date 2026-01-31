---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#111-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary:  * Bytes values are equal if they contain the same sequence of bytes.  * Struct values are equal if they have the same set of regular field labels
---

 * Bytes values are equal if they contain the same sequence of bytes.
 * Struct values are equal if they have the same set of regular field labels
   and the corresponding values are recursively equal. Only regular fields are
   considered; field order and closedness are irrelevant.
 * List values are equal if they have the same length and their corresponding
   elements are recursively equal.

For ordering comparisons (<, <=, >, >=):

 * Numeric values are ordered by their numeric value, with integer-to-float
