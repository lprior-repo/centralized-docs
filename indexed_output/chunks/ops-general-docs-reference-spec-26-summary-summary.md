---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#26-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 128
summary: little_u_value   = `\` { `#` } \"u\" hex_digit hex_digit hex_digit hex_digit . big_u_value      = `\` { `#` } \"U\" hex_digit hex_digit hex_digit hex_digit
---

little_u_value   = `\` { `#` } "u" hex_digit hex_digit hex_digit hex_digit .
big_u_value      = `\` { `#` } "U" hex_digit hex_digit hex_digit hex_digit
                           hex_digit hex_digit hex_digit hex_digit .
unicode_value    = unicode_char | little_u_value | big_u_value | escaped_char .
interpolation    = "\" { `#` } "(" Expression ")" .

string_lit       = simple_string_lit |
                   multiline_string_lit |
                   simple_bytes_lit |
                   multiline_bytes_lit |
