---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#29-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 146
summary: These examples all represent the same string:. \"日本語\"                                 // UTF-8 input text
---


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
