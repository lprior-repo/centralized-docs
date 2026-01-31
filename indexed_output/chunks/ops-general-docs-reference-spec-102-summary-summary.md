---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#102-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: denotes the element of a list or struct a indexed by x. The value x is called the index or field name, respectively
---


a[x]

denotes the element of a list or struct a indexed by x.
The value x is called the index or field name, respectively.
The following rules apply:

If a is not a struct:

 * a is a list (which need not be complete)
 * the index x unified with int must be concrete.
 * the index x is in range if 0 <= x < len(a), where only the
   explicitly defined values of an open-ended list are considered,
   otherwise it is out of range

The result of a[x] is

for a of list type:

 * the list element at index x, if x is within range
