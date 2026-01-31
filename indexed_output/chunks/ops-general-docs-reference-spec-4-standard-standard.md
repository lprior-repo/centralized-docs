---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#4-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 516
summary: OPERATORS AND PUNCTUATION. The following character sequences represent operators and punctuation:
---


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
