---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#7-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 518
summary:                    multiline_string_lit |.                    simple_bytes_lit |
---

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
but allow newline characters.

Multiline strings and byte sequences respectively start with
a triple double quote (""") or triple single quote ('''),
