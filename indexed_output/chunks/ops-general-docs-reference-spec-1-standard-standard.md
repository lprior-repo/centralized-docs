---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#1-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 514
summary: CUE draws its influence from many languages. Its main influences were BCL/GCL (internal to Google),
---


CUE draws its influence from many languages.
Its main influences were BCL/GCL (internal to Google),
LKB (LinGO), Go, and JSON.
Others are Swift, Typescript, Javascript, Prolog, NCL (internal to Google),
Jsonnet, HCL, Flabbergast, Nix, JSONPath, Haskell, Objective-C, and Python.

NOTATION

The syntax is specified using Extended Backus-Naur Form (EBNF):


Copy code
Copied!

Production  = production_name "=" [ Expression ] "." .
Expression  = Alternative { "|" Alternative } .
Alternative = Term { Term } .
Term        = production_name | token [ "…" token ] | Group | Option | Repetition .
Group       = "(" Expression ")" .
Option      = "[" Expression "]" .
Repetition  = "{" Expression "}" .

Productions are expressions constructed from terms and the following operators,
in increasing precedence:


Copy code
Copied!

|   alternation
()  grouping
[]  option (0 or 1 times)
{}  repetition (0 to n times)

Lower-case production names are used to identify lexical tokens. Non-terminals
are in CamelCase. Lexical tokens are enclosed in double quotes "" or back
quotes ``.

The form a … b represents the set of characters from a through b as
alternatives. The horizontal ellipsis … is also used elsewhere in the spec to
informally denote various enumerations or code snippets that are not further
specified. The character … (as opposed to the three characters ...) is not a
token of the CUE language.

SOURCE CODE REPRESENTATION

Source code is Unicode text encoded in UTF-8.
Unless otherwise noted, the text is not canonicalized, so a single
accented code point is distinct from the same character constructed from
combining an accent and a letter; those are treated as two code points.
For simplicity, this document will use the unqualified term character to refer
to a Unicode code point in the source text.

Each code point is distinct; for instance, upper and lower case letters are
different characters.

Implementation restriction: For compatibility with other tools, a compiler may
disallow the NUL character (U+0000) in the source text.
