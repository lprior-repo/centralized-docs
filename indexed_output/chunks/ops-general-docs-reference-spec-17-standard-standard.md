---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#17-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 524
summary: are also declared in the result of unification. Additionally, a default constraint, denoted 
---

are also declared in the result of unification.

Additionally, a default constraint, denoted ...value, defines a value
to unify with any field for which there is no other declaration in a struct.
When unifying structs a and b,
a default constraint ...v declared in a
defines that the value v should unify with any field in the resulting struct c
whose label does not unify with any of the patterns of the pattern
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
intMap: {
    t1: 43
    t2: 2.4  // error: 2.4 is not an integer
}

nameMap: [string]: {
    firstName: string
    nickName:  *firstName | string
}

nameMap: hank: firstName: "Hank"

The optional field set defined by nameMap matches every field,
in this case just hank, and unifies the associated constraint
with the matched field, resulting in:


Copy code
Copied!

nameMap: hank: {
    firstName: "Hank"
    nickName:  "Hank"
}


CLOSED STRUCTS

By default, structs are open to adding fields.
Instances of an open struct p may contain fields not defined in p.
