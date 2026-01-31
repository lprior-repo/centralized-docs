---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#73-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 140
summary: Syntactically, embeddings may be any expression. // S1 is { a: 1, b: 2, c: 3 }
---


Syntactically, embeddings may be any expression.


Copy code
Copied!

S1: {
    a: 1
    b: 2
    {
        c: 3
    }
}
// S1 is { a: 1, b: 2, c: 3 }

S2: close({
    a: 1
    b: 2
    {
        c: 3
    }
})
// same as close(S1)

S3: {
    a: 1
    b: 2
    close({
        c: 3
    })
}
// same as S2


DEFINITIONS AND HIDDEN FIELDS

A field is a definition if its identifier starts with # or _#.
A field is hidden if its identifier starts with a _.
All other fields are regular.

Definitions and hidden fields are not emitted when converting a CUE program
