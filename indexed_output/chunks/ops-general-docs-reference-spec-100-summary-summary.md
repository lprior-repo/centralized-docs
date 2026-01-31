---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#100-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 149
summary: 1415, true). For a primary expression [/docs/reference/spec/#primary-expressions] x that is not a package name [/docs/reference/spec/#package-clause],
---



Copy code
Copied!

x
2
(s + ".txt")
f(3.1415, true)
m["foo"]
obj.color
f.p[i].x

SELECTORS

For a primary expression [/docs/reference/spec/#primary-expressions] x that is not a package name [/docs/reference/spec/#package-clause],
the selector expression


Copy code
Copied!

x.f

denotes the element of a struct x identified by f.

f must be an identifier or a string literal identifying
any definition or regular non-optional field.
The identifier f is called the field selector.

If x is a package name, see the section on qualified identifiers [/docs/reference/spec/#qualified-identifiers].
