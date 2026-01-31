---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#75-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 136
summary: #MyStruct: {.     sub: field:    string
---



Copy code
Copied!

#MyStruct: {
    sub: field:    string
}

#MyStruct: {
    sub: enabled?: bool
}

myValue: #MyStruct & {
    sub: feild:   2     // error, feild not defined in #MyStruct
    sub: enabled: true  // okay
}

#D: {
    #OneOf

    c: int // adds this field.
}

#OneOf: { a: int } | { b: int }


D1: #D & { a: 12, c: 22 }  // { a: 12, c: 22 }
D2: #D & { a: 12, b: 33 }  // _|_ // cannot define both `a` and `b`


Copy code
Copied!

#A: {a: int}

B: {
    #A
    b: c: int
}

x: B
x: d: 3  // not allowed, as closed by embedded #A
