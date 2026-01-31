---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#8-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 140
summary: point in the source text.  A byte order mark may be disallowed anywhere else in
---

point in the source text. A byte order mark may be disallowed anywhere else in
the source.

CHARACTERS

The following terms are used to denote specific Unicode character classes:


Copy code
Copied!

newline        = /* the Unicode code point U+000A */ .
unicode_char   = /* an arbitrary Unicode code point except newline */ .
unicode_letter = /* a Unicode code point classified as "Letter" */ .
unicode_digit  = /* a Unicode code point classified as "Number, decimal digit" */ .

In The Unicode Standard 8.0, Section 4.5 “General Category” defines a set of
