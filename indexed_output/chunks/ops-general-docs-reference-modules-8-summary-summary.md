---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#8-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 135
summary:    (/, U+002F).  It must not begin or end with a slash
---

   (/, U+002F). It must not begin or end with a slash.
 * No characters are allowed in the path except lower case ASCII letters,
   ASCII digits, and limited ASCII punctuation (-, _, .).
 * The first character of each path element is a letter or a digit.
 * No more than one period (.) is allowed in sequence.
 * No more than two underscores (_) are allowed in sequence.

In addition, the first path element must contain at least one period character (.).

No restriction is directly enforced on the length of module names, but as registries
