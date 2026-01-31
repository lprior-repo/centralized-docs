---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#28-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 137
summary: Carriage return characters (\r) inside string literals are discarded from. the string value
---

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
