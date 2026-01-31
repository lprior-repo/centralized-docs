---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#24-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 133
summary: \a   U+0007 alert or bell. \b   U+0008 backspace
---



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
