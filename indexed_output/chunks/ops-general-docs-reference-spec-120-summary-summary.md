---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#120-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: The expression is evaluated in the scope within which the string is defined. The result of the expression is substituted as follows:
---

The expression is evaluated in the scope within which the string is defined.

The result of the expression is substituted as follows:

 * string: as is
 * bool: the JSON representation of the bool
 * number: a JSON representation of the number that preserves the
   precision of the underlying binary-coded decimal
 * bytes: as if substituted within single quotes or
   converted to valid UTF-8 replacing the
   maximal subpart of ill-formed subsequences with a single
   replacement character (W3C encoding standard) otherwise
