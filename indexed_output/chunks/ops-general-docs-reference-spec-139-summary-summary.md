---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#139-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 140
summary:     tail: #List. // Disallowed: another infinite structure (a:{b:{d:{b:{d:{
---

#List: {
    head: 1
    tail: #List
}

// Disallowed: another infinite structure (a:{b:{d:{b:{d:{...}}}}}, ...).
a: {
    b: c
}
c: {
    d: a
}

// #List defines a list of arbitrary length. Because the recursive reference
// is part of a disjunction, this does not result in a structural cycle.
#List: {
    head: _
    tail: null | #List
}

// Usage of #List. The value of tail in the most deeply nested element will
// be `null`: as the value of the disjunct referring to list is the only
// conjunct, all conjuncts are cyclic and the value is invalid and so
