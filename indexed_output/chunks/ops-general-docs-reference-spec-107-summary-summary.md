---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#107-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: *    product                integers, floats, strings, bytes. /    quotient               integers, floats
---

*    product                integers, floats, strings, bytes
/    quotient               integers, floats

For any operator that accepts operands of type float, any operand may be
of type int or float, in which case the result will be float
if it cannot be represented as an int or if any of the operands are float,
or int otherwise.
So the result of 1 / 2 is 0.5 and is of type float.

The result of division by zero is bottom (an error).

Integer division is implemented through the builtin functions
quo, rem, div, and mod.
