---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#11-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 138
summary: trigger the insertion of a comma.  While breaking the input into tokens, the
---

trigger the insertion of a comma. While breaking the input into tokens, the
next token is the longest sequence of characters that form a valid token.

COMMAS

The formal grammar uses commas , as terminators in a number of productions.
CUE programs may omit most of these commas using the following rules:

When the input is broken into tokens, a comma is automatically inserted into
the token stream immediately after a line’s final token if that token is

 * an identifier, keyword, or bottom
 * a number or string literal, including an interpolation
