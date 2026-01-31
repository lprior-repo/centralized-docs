---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#108-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 141
summary: The unary operators + and - are defined for numeric values as follows:. +x                          is 0 + x
---


The unary operators + and - are defined for numeric values as follows:


Copy code
Copied!

+x                          is 0 + x
-x    negation              is 0 - x


STRING OPERATORS

Strings can be concatenated using the + operator:


Copy code
Copied!

s: "hi " + name + " and good bye"

String addition creates a new string by concatenating the operands.

A string can be repeated by multiplying it:


Copy code
Copied!

s: "etc. "*3  // "etc. etc. etc. "


COMPARISON OPERATORS

Comparison operators compare two concrete operands and yield a boolean value.
