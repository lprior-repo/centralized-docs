---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#110-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 133
summary: For equality comparisons (== and !=):.  * Two values of different basic types are always unequal, except for integers
---

operand.

For equality comparisons (== and !=):

 * Two values of different basic types are always unequal, except for integers
   and floating-point numbers (see below).
 * Null values are equal only to other null values.
 * Boolean values are equal if they are both true or both false.
 * Numeric values are equal if they represent the same number.
   When comparing an integer with a floating-point number, the integer is first
   converted to floating-point.
 * String values are equal if they contain the same sequence of bytes.
