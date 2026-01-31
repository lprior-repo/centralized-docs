---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#5-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 525
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

A string literal represents a string constant obtained from concatenating a
sequence of characters.
Byte sequences are a sequence of bytes.

String and byte sequence literals are character sequences between,
respectively, double and single quotes, as in "bar" and 'bar'.
Within the quotes, any character may appear except newline and,
respectively, unescaped double or single quote.
String literals may only be valid UTF-8.
Byte sequences may contain any sequence of bytes.

Several escape sequences allow arbitrary values to be encoded as ASCII text.
An escape sequence starts with an escape delimiter, which is \ by default.
The escape delimiter may be altered to be \ plus a fixed number of
hash symbols # by padding the start and end of a string or byte sequence
literal with this number of hash symbols.

There are four ways to represent the integer value as a numeric constant: \x
followed by exactly two hexadecimal digits; \u followed by exactly four
hexadecimal digits; \U followed by exactly eight hexadecimal digits, and a
plain backslash \ followed by exactly three octal digits.
In each case the value of the literal is the value represented by the
digits in the corresponding base.
Hexadecimal and octal escapes are only allowed within byte sequences
(single quotes).

Although these representations all result in an integer, they have different
valid ranges.
Octal escapes must represent a value between 0 and 255 inclusive.
Hexadecimal escapes satisfy this condition by construction.
The escapes \u and \U represent Unicode code points so within them
