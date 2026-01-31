---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#25-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 141
summary: A backslash at the end of a line elides the line terminator that follows it. This may not escape the final newline inside a multiline string: that
---


A backslash at the end of a line elides the line terminator that follows it.
This may not escape the final newline inside a multiline string: that
newline is already implicitly elided.

All other sequences starting with a backslash are illegal inside literals.


Copy code
Copied!

escaped_char     = `\` { `#` } ( "a" | "b" | "f" | "n" | "r" | "t" | "v" | "/" | `\` | "'" | `"` ) .
byte_value       = octal_byte_value | hex_byte_value .
octal_byte_value = `\` { `#` } octal_digit octal_digit octal_digit .
hex_byte_value   = `\` { `#` } "x" hex_digit hex_digit .
