---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#3-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1030
summary: Hexadecimal escapes satisfy this condition by construction. The escapes \u and \U represent Unicode code points so within them
---

Hexadecimal escapes satisfy this condition by construction.
The escapes \u and \U represent Unicode code points so within them
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
                   multiline_string_lit |
                   simple_bytes_lit |
                   multiline_bytes_lit |
                   `#` string_lit `#` .

simple_string_lit    = `"` { unicode_value | interpolation } `"` .
simple_bytes_lit     = `'` { unicode_value | interpolation | byte_value } `'` .
multiline_string_lit = `"""` newline
                             { unicode_value | interpolation | newline }
                             newline `"""` .
multiline_bytes_lit  = "'''" newline
                             { unicode_value | interpolation | byte_value | newline }
                             newline "'''" .

Carriage return characters (\r) inside string literals are discarded from
the string value.


Copy code
Copied!

'a\000\xab'
'\007'
'\377'
'\xa'        // illegal: too few hexadecimal digits
"\n"
"\""
'Hello, world!\n'
"Hello, \( name )!"
"日本語"
"\u65e5本\U00008a9e"
'\xff\u00FF'
"\uD800"             // illegal: surrogate half (TODO: probably should allow)
"\U00110000"         // illegal: invalid Unicode code point

#"This is not an \(interpolation)"#
#"This is an \#(interpolation)"#
#"The sequence "\U0001F604" renders as \#U0001F604."#

These examples all represent the same string:


Copy code
Copied!

"日本語"                                 // UTF-8 input text
'日本語'                                 // UTF-8 input text as byte sequence
"\u65e5\u672c\u8a9e"                    // the explicit Unicode code points
"\U000065e5\U0000672c\U00008a9e"        // the explicit Unicode code points
'\xe6\x97\xa5\xe6\x9c\xac\xe8\xaa\x9e'  // the explicit UTF-8 bytes

If the source code represents a character as two code points, such as a
combining form involving an accent and a letter, the result will appear as two
code points if placed in a string literal.

Strings and byte sequences have a multiline equivalent.
Multiline strings are like their single-line equivalent,
