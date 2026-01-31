---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#3-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 519
summary: CUE programs may omit most of these commas using the following rules:. When the input is broken into tokens, a comma is automatically inserted into
---

CUE programs may omit most of these commas using the following rules:

When the input is broken into tokens, a comma is automatically inserted into
the token stream immediately after a line’s final token if that token is

 * an identifier, keyword, or bottom
 * a number or string literal, including an interpolation
 * one of the characters ), ], }, or ?
 * an ellipsis ...

Although commas are automatically inserted, the parser will require
explicit commas between two list elements.

To reflect idiomatic use, examples in this document elide commas using
these rules.

IDENTIFIERS

Identifiers name entities such as fields and aliases.
An identifier is a sequence of one or more letters (which includes _ and $)
and digits, optionally preceded by # or _#.
It may not be _ or $.
The first character in an identifier, or after an # if it contains one,
must be a letter.
Identifiers starting with a # or _ are reserved for definitions and hidden
fields.


Copy code
Copied!

identifier  = [ "#" | "_#" ] letter { letter | unicode_digit } .


Copy code
Copied!

a
_x9
fieldName
αβ

Some identifiers are predeclared [/docs/reference/spec/#predeclared-identifiers].

KEYWORDS

CUE has a limited set of keywords.
In addition, CUE reserves all identifiers starting with __ (double underscores)
as keywords.
These are typically targets of pre-declared identifiers.

All keywords may be used as labels (field names).
Unless noted otherwise, they can also be used as identifiers to refer to
the same name.


VALUES

The following keywords are values.


Copy code
Copied!

null         true         false

These can never be used to refer to a field of the same name.
This restriction is to ensure compatibility with JSON configuration files.


PREAMBLE

The following keywords are used at the preamble of a CUE file.
After the preamble, they may be used as identifiers to refer to namesake fields.


Copy code
Copied!

package      import


COMPREHENSION CLAUSES

The following keywords are used in comprehensions.


Copy code
Copied!

for          in           if           let
