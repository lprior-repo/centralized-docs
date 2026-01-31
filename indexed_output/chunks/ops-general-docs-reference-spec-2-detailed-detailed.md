---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#2-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1025
summary: for          in           if           let. OPERATORS AND PUNCTUATION
---



Copy code
Copied!

for          in           if           let

OPERATORS AND PUNCTUATION

The following character sequences represent operators and punctuation:


Copy code
Copied!

+     &&    ==    <     =     (     )
-     ||    !=    >     :     {     }
*     &     =~    <=    ?     [     ]     ,
/     |     !~    >=    !     _|_   ...   .

NUMERIC LITERALS

There are several kinds of numeric literals.


Copy code
Copied!

int_lit     = decimal_lit | si_lit | octal_lit | binary_lit | hex_lit .
decimal_lit = "0" | ( "1" … "9" ) { [ "_" ] decimal_digit } .
decimals    = decimal_digit { [ "_" ] decimal_digit } .
si_it       = decimals [ "." decimals ] multiplier |
              "." decimals  multiplier .
binary_lit  = "0b" binary_digit { [ "_" ] binary_digit } .
hex_lit     = "0" ( "x" | "X" ) hex_digit { [ "_" ] hex_digit } .
octal_lit   = "0o" octal_digit { [ "_" ] octal_digit } .
multiplier  = ( "K" | "M" | "G" | "T" | "P" ) [ "i" ]

float_lit   = decimals "." [ decimals ] [ exponent ] |
              decimals exponent |
              "." decimals [ exponent ].
exponent    = ( "e" | "E" ) [ "+" | "-" ] decimals .

An integer literal is a sequence of digits representing an integer value.
An optional prefix sets a non-decimal base: 0o for octal,
0x or 0X for hexadecimal, and 0b for binary.
In hexadecimal literals, letters a … f and A … F represent values 10 through 15.
All integers allow interstitial underscores _;
these have no meaning and are solely for readability.

Integer literals may have an SI or IEC multiplier.
Multipliers can be used with fractional numbers.
When multiplying a fraction by a multiplier, the result is truncated
towards zero if it is not an integer.


Copy code
Copied!

42
1.5G    // 1_500_000_000
1.3Ki   // 1.3 * 1024 = trunc(1331.2) = 1331
170_141_183_460_469_231_731_687_303_715_884_105_727
0xBad_Face
0o755
0b0101_0001

A decimal floating-point literal is a representation of
a decimal floating-point value (a float).
It has an integer part, a decimal point, a fractional part, and an
exponent part.
The integer and fractional part comprise decimal digits; the
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
