---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#19-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 133
summary: exponent part is an e or E followed by an optionally signed decimal exponent. One of the integer part or the fractional part may be elided; one of the decimal
---

exponent part is an e or E followed by an optionally signed decimal exponent.
One of the integer part or the fractional part may be elided; one of the decimal
point or the exponent may be elided.


Copy code
Copied!

0.
72.40
072.40  // == 72.40
2.71828
1.e+0
6.67428e-11
1E6
.25
.12345E+5

Neither a float_lit nor an si_lit may appear after a token that is:

 * an identifier, keyword, or bottom
 * a number or string literal, including an interpolation
 * one of the characters ), ], }, ?, or ..

STRING AND BYTE SEQUENCE LITERALS
