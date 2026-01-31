---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#24-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 512
summary: 797693134862315708145274237317043567981e+308. EXPORTED IDENTIFIERS
---

          <=1.797693134862315708145274237317043567981e+308

EXPORTED IDENTIFIERS

An identifier of a package may be exported to permit access to it
from another package.
All identifiers not starting with _ (so all regular fields and definitions
starting with #) are exported.
Any identifier starting with _ is not visible outside the package and resides
in a separate namespace than namesake identifiers of other packages.


Copy code
Copied!

package mypackage

foo:   string  // visible outside mypackage
"bar": string  // visible outside mypackage

#Foo: {      // visible outside mypackage
    a:  1    // visible outside mypackage
    _b: 2    // not visible outside mypackage

    #C: {    // visible outside mypackage
        d: 4 // visible outside mypackage
    }
    _#E: foo // not visible outside mypackage
}

UNIQUENESS OF IDENTIFIERS

Given a set of identifiers, an identifier is called unique if it is different
from every other in the set, after applying normalization following
Unicode Annex #31 [https://unicode.org/reports/tr31/].
Two identifiers are different if they are spelled differently
or if they appear in different packages and are not exported.
Otherwise, they are the same.

FIELD DECLARATIONS

A field associates the value of an expression to a label within a struct.
If this label is an identifier, it binds the field to that identifier,
so the field’s value can be referenced by writing the identifier.
String labels are not bound to fields.


Copy code
Copied!

a: {
    b: 2
    "s": 3

    c: b   // 2
    d: s   // _|_ unresolved identifier "s"
    e: a.s // 3
}

If an expression may result in a value associated with a default value
as described in default values [/docs/reference/spec/#default-values], the field binds to this
value-default pair.

LET DECLARATIONS

Within a struct, a let clause binds an identifier to the given expression.

Within the scope of the identifier, the identifier refers to the
locally declared expression.
The expression is evaluated in the scope it was declared.

EXPRESSIONS
