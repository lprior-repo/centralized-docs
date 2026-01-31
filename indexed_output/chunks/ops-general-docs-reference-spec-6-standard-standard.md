---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#6-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 518
summary: some values are illegal, in particular those above 0x10FFFF. Surrogate halves are allowed,
---

some values are illegal, in particular those above 0x10FFFF.
Surrogate halves are allowed,
but are translated into their non-surrogate equivalent internally.

The three-digit octal (\nnn) and two-digit hexadecimal (\xnn) escapes
represent individual bytes of the resulting string; all other escapes represent
the (possibly multi-byte) UTF-8 encoding of individual characters.
Thus inside a string literal \377 and \xFF represent a single byte of
value 0xFF=255, while ÿ, \u00FF, \U000000FF and \xc3\xbf represent
the two bytes 0xc3 0xbf of the UTF-8 encoding of character U+00FF.


Copy code
Copied!

\a   U+0007 alert or bell
\b   U+0008 backspace
\f   U+000C form feed
\n   U+000A line feed or newline
\r   U+000D carriage return
\t   U+0009 horizontal tab
\v   U+000b vertical tab
\/   U+002f slash (solidus)
\\   U+005c backslash
\'   U+0027 single quote  (valid escape only within single quoted literals)
\"   U+0022 double quote  (valid escape only within double quoted literals)

The escape \( is used as an escape for string interpolation.
A \( must be followed by a valid CUE Expression, followed by a ).

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
little_u_value   = `\` { `#` } "u" hex_digit hex_digit hex_digit hex_digit .
big_u_value      = `\` { `#` } "U" hex_digit hex_digit hex_digit hex_digit
                           hex_digit hex_digit hex_digit hex_digit .
unicode_value    = unicode_char | little_u_value | big_u_value | escaped_char .
interpolation    = "\" { `#` } "(" Expression ")" .

string_lit       = simple_string_lit |
