---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#76-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 136
summary: y: d: 3  // allowed as nothing closes b. z: d: 3  // not allowed, as referencing #B closes b
---


y: B.b
y: d: 3  // allowed as nothing closes b

#B: {
    #A
    b: c: int
}

z: #B.b
z: d: 3  // not allowed, as referencing #B closes b


ATTRIBUTES

Attributes allow associating meta information with values.
Their primary purpose is to define mappings between CUE and
other representations.
Attributes do not influence the evaluation of CUE.

An attribute associates an identifier with a value, a balanced token sequence,
which is a sequence of CUE tokens with balanced brackets ((), [], and {}).
The sequence may not contain interpolations.
