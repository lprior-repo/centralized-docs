---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#4-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 148
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
