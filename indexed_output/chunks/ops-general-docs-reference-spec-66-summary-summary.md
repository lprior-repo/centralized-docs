---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#66-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 149
summary: constraints defined for a and for which there exists no field declaration. in a with that label
---

constraints defined for a and for which there exists no field declaration
in a with that label.
The token ... is a shorthand for ..._.
Note: default constraints of the form ..._ are not yet implemented.


Copy code
Copied!

a: {
    foo:      string  // foo is a string
    [=~"^i"]: int     // all other fields starting with i are integers
    [=~"^b"]: bool    // all other fields starting with b are booleans
    [>"c"]:   string  // all other fields lexically after c are strings

    ...string         // all other fields must be a string. Note: default constraints are not yet implemented.
