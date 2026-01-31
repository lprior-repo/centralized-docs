---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#27-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 128
summary:                    `#` string_lit `#` . simple_string_lit    = `\"` { unicode_value | interpolation } `\"` 
---

                   `#` string_lit `#` .

simple_string_lit    = `"` { unicode_value | interpolation } `"` .
simple_bytes_lit     = `'` { unicode_value | interpolation | byte_value } `'` .
multiline_string_lit = `"""` newline
                             { unicode_value | interpolation | newline }
                             newline `"""` .
multiline_bytes_lit  = "'''" newline
                             { unicode_value | interpolation | byte_value | newline }
                             newline "'''" .

