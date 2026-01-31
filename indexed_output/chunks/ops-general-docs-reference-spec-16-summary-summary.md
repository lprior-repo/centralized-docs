---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#16-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 135
summary: decimal_lit = \"0\" | ( \"1\" … \"9\" ) { [ \"_\" ] decimal_digit } . decimals    = decimal_digit { [ \"_\" ] decimal_digit } 
---

decimal_lit = "0" | ( "1" … "9" ) { [ "_" ] decimal_digit } .
decimals    = decimal_digit { [ "_" ] decimal_digit } .
si_it       = decimals [ "." decimals ] multiplier |
              "." decimals  multiplier .
binary_lit  = "0b" binary_digit { [ "_" ] binary_digit } .
hex_lit     = "0" ( "x" | "X" ) hex_digit { [ "_" ] hex_digit } .
octal_lit   = "0o" octal_digit { [ "_" ] octal_digit } .
multiplier  = ( "K" | "M" | "G" | "T" | "P" ) [ "i" ]

float_lit   = decimals "." [ decimals ] [ exponent ] |
              decimals exponent |
