---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#20-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 138
summary: A string literal represents a string constant obtained from concatenating a. sequence of characters
---


A string literal represents a string constant obtained from concatenating a
sequence of characters.
Byte sequences are a sequence of bytes.

String and byte sequence literals are character sequences between,
respectively, double and single quotes, as in "bar" and 'bar'.
Within the quotes, any character may appear except newline and,
respectively, unescaped double or single quote.
String literals may only be valid UTF-8.
Byte sequences may contain any sequence of bytes.

Several escape sequences allow arbitrary values to be encoded as ASCII text.
