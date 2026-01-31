---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#67-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary:     bar:   true.     other: \"a string\"
---

}

b: a & {
    i3:    3
    bar:   true
    other: "a string"
}

Concrete field labels may be an identifier or string, the latter of which may be
interpolated.
Fields with identifier labels can be referred to within the scope they are
defined, string labels cannot.
References within such interpolated strings are resolved within
the scope of the struct in which the label sequence is
defined and can reference concrete labels lexically preceding
the label within a label sequence.


Copy code
Copied!

intMap: [string]: int
