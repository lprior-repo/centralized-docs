---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#23-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: Surrogate halves are allowed,. but are translated into their non-surrogate equivalent internally
---

Surrogate halves are allowed,
but are translated into their non-surrogate equivalent internally.

The three-digit octal (\nnn) and two-digit hexadecimal (\xnn) escapes
represent individual bytes of the resulting string; all other escapes represent
the (possibly multi-byte) UTF-8 encoding of individual characters.
Thus inside a string literal \377 and \xFF represent a single byte of
value 0xFF=255, while ÿ, \u00FF, \U000000FF and \xc3\xbf represent
the two bytes 0xc3 0xbf of the UTF-8 encoding of character U+00FF.
