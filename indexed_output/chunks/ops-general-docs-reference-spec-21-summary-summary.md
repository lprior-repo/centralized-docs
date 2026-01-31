---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#21-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 135
summary: An escape sequence starts with an escape delimiter, which is \ by default. The escape delimiter may be altered to be \ plus a fixed number of
---

An escape sequence starts with an escape delimiter, which is \ by default.
The escape delimiter may be altered to be \ plus a fixed number of
hash symbols # by padding the start and end of a string or byte sequence
literal with this number of hash symbols.

There are four ways to represent the integer value as a numeric constant: \x
followed by exactly two hexadecimal digits; \u followed by exactly four
hexadecimal digits; \U followed by exactly eight hexadecimal digits, and a
plain backslash \ followed by exactly three octal digits.
