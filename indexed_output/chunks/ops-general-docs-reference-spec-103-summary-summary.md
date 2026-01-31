---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#103-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary:  * bottom (an error), otherwise. for a of struct type:
---

 * bottom (an error), otherwise

for a of struct type:

 * the index x unified with string must be concrete.
 * the value of the regular and non-optional field named x of struct a,
   if this field exists
 * bottom (an error), otherwise


Copy code
Copied!

a: [ 1, 2 ][1]     // 2
b: [ 1, 2 ][2]     // _|_
c: [ 1, 2, ...][2] // _|_

// Defaults are selected for both operand and index:
x: [1, 2] | *[3, 4]
y: int | *1
z: x[y]  // 4

OPERATORS

Operators combine operands into expressions.


Copy code
Copied!

