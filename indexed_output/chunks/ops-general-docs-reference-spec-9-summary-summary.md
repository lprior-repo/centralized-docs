---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#9-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 135
summary: character categories. CUE treats all characters in any of the Letter categories Lu, Ll, Lt, Lm, or Lo
---

character categories.
CUE treats all characters in any of the Letter categories Lu, Ll, Lt, Lm, or Lo
as Unicode letters, and those in the Number category Nd as Unicode digits.

LETTERS AND DIGITS

The underscore character _ (U+005F) is considered a letter.


Copy code
Copied!

letter        = unicode_letter | "_" | "$" .
decimal_digit = "0" … "9" .
binary_digit  = "0" … "1" .
octal_digit   = "0" … "7" .
hex_digit     = "0" … "9" | "A" … "F" | "a" … "f" .

LEXICAL ELEMENTS

COMMENTS

Comments serve as program documentation.
