---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#68-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 140
summary: 4  // error: 2. 4 is not an integer
---

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
