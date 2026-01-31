---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#36-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 516
summary: If a node a references an ancestor node, we call it and any of its. field values a
---


If a node a references an ancestor node, we call it and any of its
field values a.f cyclic.
So if a is cyclic, all of its descendants are also regarded as cyclic.
A given node x, whose value is composed of the conjuncts c1 & ... & cn,
is valid if any of its conjuncts is not cyclic.


Copy code
Copied!

// Disallowed: a list of infinite length with all elements being 1.
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
// eliminated from the disjunction.
MyList: #List & { head: 1, tail: { head: 2 }}

MODULES, INSTANCES, AND PACKAGES

CUE configurations are constructed combining instances.
An instance, in turn, is constructed from one or more source files belonging
to the same package that together declare the data representation.
Elements of this data representation may be exported and used
in other instances.

SOURCE FILE ORGANIZATION

Each source file consists of an optional package clause defining collection
of files to which it belongs,
followed by a possibly empty set of import declarations that declare
packages whose contents it wishes to use, followed by a possibly empty set of
declarations.

Like with a struct, a source file may contain embeddings.
Unlike with a struct, the embedded expressions may be any value.
If the result of the unification of all embedded values is not a struct,
it will be output instead of its enclosing file when exporting CUE
to a data format


Copy code
Copied!

SourceFile = { attribute "," } [ PackageClause "," ] { ImportDecl "," } { Declaration "," } .


Copy code
Copied!

"Hello \(#place)!"
