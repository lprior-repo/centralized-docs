---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#0-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1033
summary: # The CUE Language Specification | CUE. **Source:** https://cuelang
---

# The CUE Language Specification | CUE

**Source:** https://cuelang.org/docs/reference/spec/

Skip to content

Homepage of CUE [/]
 * Documentation [/docs/]
 * Play [/play/]
 * Community [/community/]

 * 
   GitHub [https://github.com/cue-lang/cue]
 * 
   Slack [/s/slack]
 * 
   Discord [/s/discord]
 * 
   X (Twitter) [https://twitter.com/cue_lang]
 * 
   Bluesky [https://bsky.app/profile/cuelang.org]
 * 
   YouTube [https://www.youtube.com/@cuelang/videos]

Install
[/docs/introduction/installation/]

Search [/search]

What are you looking for?

Menu

 1. References [https://cuelang.org/docs/reference/]


 2. THE CUE LANGUAGE SPECIFICATION

mpvl [https://github.com/mpvl.png]
Marcel van Lohuizen
mpvl [https://github.com/mpvl.png]
Marcel van Lohuizen

Github profile

[https://github.com/mpvl]

Search all content by this author

[/search/?q=author:mpvl]


NOTE TO IMPLEMENTORS

Notes on the formalism underlying this specification can be found
here [https://github.com/cue-lang/cue/blob/master/doc/ref/impl.md].

INTRODUCTION

This is a reference manual for the CUE data constraint language.
CUE, pronounced cue or Q, is a general-purpose and strongly typed
constraint-based language.
It can be used for data templating, data validation, code generation, scripting,
and many other applications involving structured data.
The CUE tooling, layered on top of CUE, provides
a general purpose scripting language for creating scripts as well as
simple servers, also expressed in CUE.

CUE was designed with cloud configuration and related systems in mind,
but is not limited to this domain.
It derives its formalism from relational programming languages.
This formalism allows for managing and reasoning over large amounts of
data in a straightforward manner.

The grammar is compact and regular, allowing for easy analysis by automatic
tools such as integrated development environments.

This document is maintained by mpvl@golang.org [mpvl@golang.org].
CUE has a lot of similarities with the Go language. This document draws heavily
from the Go specification as a result.

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
