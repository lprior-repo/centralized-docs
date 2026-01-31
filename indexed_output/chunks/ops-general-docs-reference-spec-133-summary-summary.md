---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#133-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: in e have been resolved. // Config            Evaluates to (requiring concrete values)
---

in e have been resolved.


Copy code
Copied!

// Config            Evaluates to (requiring concrete values)
x: {                  x: {
    a: b + 100            a: _|_ // cycle detected
    b: a - 100            b: _|_ // cycle detected
}                     }

y: x & {              y: {
    a: 200                a: 200 // asserted that 200 == b + 100
                          b: 100
}                     }


FIELD VALUES

A field value of the form r & v,
where r evaluates to a reference cycle and v is a concrete value,
