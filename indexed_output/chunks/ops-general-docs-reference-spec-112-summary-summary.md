---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#112-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary:    conversion as described above.  * String values are ordered lexically byte-wise
---

   conversion as described above.
 * String values are ordered lexically byte-wise.
 * Bytes values are ordered lexically byte-wise.

For pattern matching (=~, !~):

 * The regular expression syntax is that accepted by RE2 (https://github.com/google/re2/wiki/Syntax) [https://github.com/google/re2/wiki/Syntax%29], except for \C.
 * s =~ r is true if string s matches regular expression r.
 * s !~ r is true if string s does not match regular expression r.


Copy code
Copied!

3 < 4       // true
3 < 4.0     // true
