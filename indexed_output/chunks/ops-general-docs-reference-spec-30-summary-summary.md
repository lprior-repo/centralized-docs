---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#30-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 146
summary: code points if placed in a string literal. Strings and byte sequences have a multiline equivalent
---

code points if placed in a string literal.

Strings and byte sequences have a multiline equivalent.
Multiline strings are like their single-line equivalent,
but allow newline characters.

Multiline strings and byte sequences respectively start with
a triple double quote (""") or triple single quote ('''),
immediately followed by a newline, which is discarded from the string contents.
The string is closed by a matching triple quote, which must be by itself
on a new line, preceded by optional whitespace.
The newline preceding the closing quote is discarded from the string contents.
