---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#10-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 147
summary: CUE supports line comments that start with the character sequence //. and stop at the end of the line
---

CUE supports line comments that start with the character sequence //
and stop at the end of the line.

A comment cannot start inside a string literal or inside a comment.
A comment acts like a newline.

TOKENS

Tokens form the vocabulary of the CUE language. There are four classes:
identifiers, keywords, operators and punctuation, and literals. White space,
formed from spaces (U+0020), horizontal tabs (U+0009), carriage returns
(U+000D), and newlines (U+000A), is ignored except as it separates tokens that
would otherwise combine into a single token. Also, a newline or end of file may
