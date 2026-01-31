---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#17-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 139
summary: \" decimals [ exponent ]. exponent    = ( \"e\" | \"E\" ) [ \"+\" | \"-\" ] decimals 
---

              "." decimals [ exponent ].
exponent    = ( "e" | "E" ) [ "+" | "-" ] decimals .

An integer literal is a sequence of digits representing an integer value.
An optional prefix sets a non-decimal base: 0o for octal,
0x or 0X for hexadecimal, and 0b for binary.
In hexadecimal literals, letters a … f and A … F represent values 10 through 15.
All integers allow interstitial underscores _;
these have no meaning and are solely for readability.

Integer literals may have an SI or IEC multiplier.
Multipliers can be used with fractional numbers.
