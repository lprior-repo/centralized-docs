---
doc_id: ops/general/docs-tour-references
chunk_id: ops/general/docs-tour-references#1-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 136
summary: What are you looking for?.  Tour [https://cuelang
---


What are you looking for?

Menu

 1. Tour [https://cuelang.org/docs/tour/]
 2. References and Visibility [https://cuelang.org/docs/tour/references/]


 3. REFERENCES AND SCOPES

A reference refers to the value of the field defined within the nearest
enclosing scope.

If a reference doesn’t match a field within the same file,
then it may match a top-level field defined in any other file making up the
same CUE package.

If there is still no match then it may match a predefined value, such as a
predefined bound [/docs/tour/types/bounddef/].
