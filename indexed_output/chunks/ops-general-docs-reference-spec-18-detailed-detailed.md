---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#18-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1036
summary: It is possible to construct a structural cycle by unifying two acyclic values:. // introduces structural cycle
---

It is possible to construct a structural cycle by unifying two acyclic values:


Copy code
Copied!

// acyclic
y: {
    f: h: g
    g: _
}
// acyclic
x: {
    f: _
    g: f
}
// introduces structural cycle
z: x & y

Implementations should be able to detect such structural cycles dynamically.

A structural cycle can result in infinite structure or evaluation loops.


Copy code
Copied!

// infinite structure
a: b: a

// infinite evaluation
f: {
    n:   int
    out: n + (f & {n: 1}).out
}

CUE must allow or disallow structural cycles under certain circumstances.

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

#place: "world"

// Outputs "Hello world!"

PACKAGE CLAUSE

A package clause is an optional clause that defines the package to which
a source file the file belongs.


Copy code
Copied!

PackageClause  = "package" PackageName .
PackageName    = identifier .

The PackageName must not be a definition identifier.

If the PackageName is the blank identifier (_), it is treated the same
as if there were no package clause. This can be useful to allow adding
package level attributes or doc comments to a CUE file without a package
name.


Copy code
Copied!

package math

MODULES AND INSTANCES

A module defines a tree of directories, rooted at the module root.

All source files within a module with the same package name belong to the same
package.

A module may define multiple packages.

An instance of a package is any subset of files belonging
to the same package.

It is interpreted as the concatenation of these files.

An implementation may impose conventions on the layout of package files
to determine which files of a package belongs to an instance.
For example, an instance may be defined as the subset of package files
belonging to a directory and all its ancestors.

IMPORT DECLARATIONS

An import declaration states that the source file containing the declaration
depends on definitions of the imported package
and enables access to exported identifiers of that package.
The import names an identifier (PackageName) to be used for access and an
ImportPath that specifies the package to be imported.
