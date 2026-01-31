---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#7-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary: For simplicity, this document will use the unqualified term character to refer. to a Unicode code point in the source text
---

For simplicity, this document will use the unqualified term character to refer
to a Unicode code point in the source text.

Each code point is distinct; for instance, upper and lower case letters are
different characters.

Implementation restriction: For compatibility with other tools, a compiler may
disallow the NUL character (U+0000) in the source text.

Implementation restriction: For compatibility with other tools, a compiler may
ignore a UTF-8-encoded byte order mark (U+FEFF) if it is the first Unicode code
