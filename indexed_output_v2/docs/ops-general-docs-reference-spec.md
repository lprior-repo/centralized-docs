---
id: ops/general/docs-reference-spec
title: Docs Reference Spec
category: ops
tags: ["language", "ops", "specification"]
---

# The CUE Language Specification | CUE

> **Context**: **Source:** https://cuelang.org/docs/reference/spec/


**Source:** https://cuelang.org/docs/reference/spec/

Skip to content

Homepage of CUE [/]
 * Documentation [/docs/]
 * Play [/play/]
 * Community [/community/]

 * 
   GitHub [https://github.com/cue-lang/cue]
 * 
   Slack [/s/slack]
 * 
   Discord [/s/discord]
 * 
   X (Twitter) [https://twitter.com/cue_lang]
 * 
   Bluesky [https://bsky.app/profile/cuelang.org]
 * 
   YouTube [https://www.youtube.com/@cuelang/videos]

Install
[/docs/introduction/installation/]

Search [/search]

What are you looking for?

Menu

 1. References [https://cuelang.org/docs/reference/]


 2. THE CUE LANGUAGE SPECIFICATION

mpvl [https://github.com/mpvl.png]
Marcel van Lohuizen
mpvl [https://github.com/mpvl.png]
Marcel van Lohuizen

Github profile

[https://github.com/mpvl]

Search all content by this author

[/search/?q=author:mpvl]


NOTE TO IMPLEMENTORS

Notes on the formalism underlying this specification can be found
here [https://github.com/cue-lang/cue/blob/master/doc/ref/impl.md].

INTRODUCTION

This is a reference manual for the CUE data constraint language.
CUE, pronounced cue or Q, is a general-purpose and strongly typed
constraint-based language.
It can be used for data templating, data validation, code generation, scripting,
and many other applications involving structured data.
The CUE tooling, layered on top of CUE, provides
a general purpose scripting language for creating scripts as well as
simple servers, also expressed in CUE.

CUE was designed with cloud configuration and related systems in mind,
but is not limited to this domain.
It derives its formalism from relational programming languages.
This formalism allows for managing and reasoning over large amounts of
data in a straightforward manner.

The grammar is compact and regular, allowing for easy analysis by automatic
tools such as integrated development environments.

This document is maintained by mpvl@golang.org [mpvl@golang.org].
CUE has a lot of similarities with the Go language. This document draws heavily
from the Go specification as a result.

CUE draws its influence from many languages.
Its main influences were BCL/GCL (internal to Google),
LKB (LinGO), Go, and JSON.
Others are Swift, Typescript, Javascript, Prolog, NCL (internal to Google),
Jsonnet, HCL, Flabbergast, Nix, JSONPath, Haskell, Objective-C, and Python.

NOTATION

The syntax is specified using Extended Backus-Naur Form (EBNF):


Copy code
Copied!

Production  = production_name "=" [ Expression ] "." .
Expression  = Alternative { "|" Alternative } .
Alternative = Term { Term } .
Term        = production_name | token [ "…" token ] | Group | Option | Repetition .
Group       = "(" Expression ")" .
Option      = "[" Expression "]" .
Repetition  = "{" Expression "}" .

Productions are expressions constructed from terms and the following operators,
in increasing precedence:


Copy code
Copied!

|   alternation
()  grouping
[]  option (0 or 1 times)
{}  repetition (0 to n times)

Lower-case production names are used to identify lexical tokens. Non-terminals
are in CamelCase. Lexical tokens are enclosed in double quotes "" or back
quotes ``.

The form a … b represents the set of characters from a through b as
alternatives. The horizontal ellipsis … is also used elsewhere in the spec to
informally denote various enumerations or code snippets that are not further
specified. The character … (as opposed to the three characters ...) is not a
token of the CUE language.

SOURCE CODE REPRESENTATION

Source code is Unicode text encoded in UTF-8.
Unless otherwise noted, the text is not canonicalized, so a single
accented code point is distinct from the same character constructed from
combining an accent and a letter; those are treated as two code points.
For simplicity, this document will use the unqualified term character to refer
to a Unicode code point in the source text.

Each code point is distinct; for instance, upper and lower case letters are
different characters.

Implementation restriction: For compatibility with other tools, a compiler may
disallow the NUL character (U+0000) in the source text.

Implementation restriction: For compatibility with other tools, a compiler may
ignore a UTF-8-encoded byte order mark (U+FEFF) if it is the first Unicode code
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
CUE supports line comments that start with the character sequence //
and stop at the end of the line.

A comment cannot start inside a string literal or inside a comment.
A comment acts like a newline.

TOKENS

Tokens form the vocabulary of the CUE language. There are four classes:
identifiers, keywords, operators and punctuation, and literals. White space,
formed from spaces (U+0020), horizontal tabs (U+0009), carriage returns
(U+000D), and newlines (U+000A), is ignored except as it separates tokens that
would otherwise combine into a single token. Also, a newline or end of file may
trigger the insertion of a comma. While breaking the input into tokens, the
next token is the longest sequence of characters that form a valid token.

COMMAS

The formal grammar uses commas , as terminators in a number of productions.
CUE programs may omit most of these commas using the following rules:

When the input is broken into tokens, a comma is automatically inserted into
the token stream immediately after a line’s final token if that token is

 * an identifier, keyword, or bottom
 * a number or string literal, including an interpolation
 * one of the characters ), ], }, or ?
 * an ellipsis ...

Although commas are automatically inserted, the parser will require
explicit commas between two list elements.

To reflect idiomatic use, examples in this document elide commas using
these rules.

IDENTIFIERS

Identifiers name entities such as fields and aliases.
An identifier is a sequence of one or more letters (which includes _ and $)
and digits, optionally preceded by # or _#.
It may not be _ or $.
The first character in an identifier, or after an # if it contains one,
must be a letter.
Identifiers starting with a # or _ are reserved for definitions and hidden
fields.


Copy code
Copied!

identifier  = [ "#" | "_#" ] letter { letter | unicode_digit } .


Copy code
Copied!

a
_x9
fieldName
αβ

Some identifiers are predeclared [/docs/reference/spec/#predeclared-identifiers].

KEYWORDS

CUE has a limited set of keywords.
In addition, CUE reserves all identifiers starting with __ (double underscores)
as keywords.
These are typically targets of pre-declared identifiers.

All keywords may be used as labels (field names).
Unless noted otherwise, they can also be used as identifiers to refer to
the same name.


VALUES

The following keywords are values.


Copy code
Copied!

null         true         false

These can never be used to refer to a field of the same name.
This restriction is to ensure compatibility with JSON configuration files.


PREAMBLE

The following keywords are used at the preamble of a CUE file.
After the preamble, they may be used as identifiers to refer to namesake fields.


Copy code
Copied!

package      import


COMPREHENSION CLAUSES

The following keywords are used in comprehensions.


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
Hexadecimal escapes satisfy this condition by construction.
The escapes \u and \U represent Unicode code points so within them
some values are illegal, in particular those above 0x10FFFF.
Surrogate halves are allowed,
but are translated into their non-surrogate equivalent internally.

The three-digit octal (\nnn) and two-digit hexadecimal (\xnn) escapes
represent individual bytes of the resulting string; all other escapes represent
the (possibly multi-byte) UTF-8 encoding of individual characters.
Thus inside a string literal \377 and \xFF represent a single byte of
value 0xFF=255, while ÿ, \u00FF, \U000000FF and \xc3\xbf represent
the two bytes 0xc3 0xbf of the UTF-8 encoding of character U+00FF.


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

A backslash at the end of a line elides the line terminator that follows it.
This may not escape the final newline inside a multiline string: that
newline is already implicitly elided.

All other sequences starting with a backslash are illegal inside literals.


Copy code
Copied!

escaped_char     = `\` { `#` } ( "a" | "b" | "f" | "n" | "r" | "t" | "v" | "/" | `\` | "'" | `"` ) .
byte_value       = octal_byte_value | hex_byte_value .
octal_byte_value = `\` { `#` } octal_digit octal_digit octal_digit .
hex_byte_value   = `\` { `#` } "x" hex_digit hex_digit .
little_u_value   = `\` { `#` } "u" hex_digit hex_digit hex_digit hex_digit .
big_u_value      = `\` { `#` } "U" hex_digit hex_digit hex_digit hex_digit
                           hex_digit hex_digit hex_digit hex_digit .
unicode_value    = unicode_char | little_u_value | big_u_value | escaped_char .
interpolation    = "\" { `#` } "(" Expression ")" .

string_lit       = simple_string_lit |
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
immediately followed by a newline, which is discarded from the string contents.
The string is closed by a matching triple quote, which must be by itself
on a new line, preceded by optional whitespace.
The newline preceding the closing quote is discarded from the string contents.
The whitespace before a closing triple quote must appear before any non-empty
line after the opening quote and will be removed from each of these
lines in the string literal.
A closing triple quote may not appear in the string.
To include it is suffices to escape one of the quotes.


Copy code
Copied!

"""
    lily:
    out of the water
    out of itself

    bass
    picking \
    bugs
    off the moon
        — Nick Virgilio, Selected Haiku, 1988
    """

This represents the same string as:


Copy code
Copied!

"lily:\nout of the water\nout of itself\n\n" +
"bass\npicking bugs\noff the moon\n" +
"    — Nick Virgilio, Selected Haiku, 1988"

VALUES

In addition to simple values like "hello" and 42.0, CUE has structs [/docs/reference/spec/#structs].
A struct is a map from labels to values, like {a: 42.0, b: "hello"}.
Structs are CUE’s only way of building up complex values;
lists, which we will see later,
are defined in terms of structs.

All possible values are ordered in a lattice,
a partial order where every two elements have a single greatest lower bound.
A value a is an instance of a value b,
denoted a ⊑ b, if b == a or b is more general than a,
that is if a orders before b in the partial order
(⊑ is not a CUE operator).
We also say that b subsumes a in this case.
In graphical terms, b is “above” a in the lattice.

At the top of the lattice is the single ancestor of all values, called
top [/docs/reference/spec/#top], denoted _ in CUE.
Every value is an instance of top.

At the bottom of the lattice is the value called bottom [/docs/reference/spec/#bottom], denoted _|_.
A bottom value usually indicates an error.
Bottom is an instance of every value.

An atom is any value whose only instances are itself and bottom.
Examples of atoms are 42.0, "hello", true, and null.

A value is concrete if it is either an atom, or a struct whose field values
of regular (non-hidden and non-definition fields) are all concrete, recursively.

CUE’s values also include what we normally think of as types, like string and
float.
It does not distinguish between types and values:
only the relationship of values in the lattice is important.
Each CUE “type” subsumes the concrete values that one would normally think
of as part of that type.
For example, "hello" is an instance of string, and 42.0 is an instance of
float.
In addition to string and float, CUE has null, int, bool, and bytes.
We informally call these CUE’s “basic types”.


Copy code
Copied!

false ⊑ bool
true  ⊑ bool
true  ⊑ true
5.0   ⊑ float
bool  ⊑ _
_|_   ⊑ _
_|_   ⊑ _|_

_     ⋢ _|_
_     ⋢ bool
int   ⋢ bool
bool  ⋢ int
false ⋢ true
true  ⋢ false
float ⋢ 5.0
5     ⋢ 6

UNIFICATION

The unification of values a and b
is defined as the greatest lower bound of a and b. (That is, the
value u such that u ⊑ a and u ⊑ b,
and for any other value v for which v ⊑ a and v ⊑ b
it holds that v ⊑ u.)
Since CUE values form a lattice, the unification of two CUE values is
always unique.

These all follow from the definition of unification:

 * The unification of a with itself is always a.
 * The unification of values a and b where a ⊑ b is always a.
 * The unification of a value with bottom is always bottom.

Unification in CUE is a binary expression [/docs/reference/spec/#operands], written a & b.
It is commutative, associative, and idempotent.
As a consequence, order of evaluation is irrelevant, a property that is key
to many of the constructs in the CUE language as well as the tooling layered
on top of it.

DISJUNCTION

The disjunction of values a and b
is defined as the least upper bound of a and b.
(That is, the value d such that a ⊑ d and b ⊑ d,
and for any other value e for which a ⊑ e and b ⊑ e,
it holds that d ⊑ e.)
This style of disjunctions is sometimes also referred to as sum types.
Since CUE values form a lattice, the disjunction of two CUE values is always unique.

These all follow from the definition of disjunction:

 * The disjunction of a with itself is always a.
 * The disjunction of a value a and b where a ⊑ b is always b.
 * The disjunction of a value a with bottom is always a.
 * The disjunction of two bottom values is bottom.

Disjunction in CUE is a binary expression [/docs/reference/spec/#operands], written a | b.
It is commutative, associative, and idempotent.

The unification of a disjunction with another value is equal to the disjunction
composed of the unification of this value with all of the original elements
of the disjunction.
In other words, unification distributes over disjunction.


Copy code
Copied!

(a_0 | ... |a_n) & b ==> a_0&b | ... | a_n&b.


Copy code
Copied!

Expression                Result
({a:1} | {b:2}) & {c:3}   {a:1, c:3} | {b:2, c:3}
(int | string) & "foo"    "foo"
("a" | "b") & "c"         _|_

A disjunction is normalized if there is no element
a for which there is an element b such that a ⊑ b.


DEFAULT VALUES

Any value v may be associated with a default value d,
where d must be in instance of v (d ⊑ v).

Default values are introduced by means of disjunctions.
Any element of a disjunction can be marked as a default
by prefixing it with an asterisk * (a unary expression [/docs/reference/spec/#operators]).
Syntactically consecutive disjunctions are considered to be
part of a single disjunction,
whereby multiple disjuncts can be marked as default.
A marked disjunction is one where any of its terms are marked.
So a | b | *c | d is a single marked disjunction of four terms,
whereas a | (b | *c | d) is an unmarked disjunction of two terms,
one of which is a marked disjunction of three terms.
During unification, if all the marked disjuncts of a marked disjunction are
eliminated, then the remaining unmarked disjuncts are considered as if they
originated from an unmarked disjunction

As explained below, distinguishing the nesting of disjunctions like this
is only relevant when both an outer and nested disjunction are marked.

Intuitively, when an expression needs to be resolved for an operation other
than unification or disjunction,
non-starred elements are dropped in favor of starred ones if the starred ones
do not resolve to bottom.

To define the unification and disjunction operation we use the notation
⟨v⟩ to denote a CUE value v that is not associated with a default
and the notation ⟨v, d⟩ to denote a value v associated with a default
value d.

The rewrite rules for unifying such values are as follows:


Copy code
Copied!

U0: ⟨v1⟩ & ⟨v2⟩         => ⟨v1&v2⟩
U1: ⟨v1, d1⟩ & ⟨v2⟩     => ⟨v1&v2, d1&v2⟩
U2: ⟨v1, d1⟩ & ⟨v2, d2⟩ => ⟨v1&v2, d1&d2⟩

The rewrite rules for disjoining terms of unmarked disjunctions are


Copy code
Copied!

D0: ⟨v1⟩ | ⟨v2⟩         => ⟨v1|v2⟩
D1: ⟨v1, d1⟩ | ⟨v2⟩     => ⟨v1|v2, d1⟩
D2: ⟨v1, d1⟩ | ⟨v2, d2⟩ => ⟨v1|v2, d1|d2⟩

Terms of marked disjunctions are first rewritten according to the following
rules:


Copy code
Copied!

M0:  ⟨v⟩    => ⟨v⟩        don't introduce defaults for unmarked term
M1: *⟨v⟩    => ⟨v, v⟩     introduce identical default for marked term
M2: *⟨v, d⟩ => ⟨v, d⟩     keep existing defaults for marked term
M3:  ⟨v, d⟩ => ⟨v⟩        strip existing defaults from unmarked term

Note that for any marked disjunction a,
the expressions a|a, *a|a and *a|*a all resolve to a.


Copy code
Copied!

Expression               Value-default pair     Rules applied
*"tcp" | "udp"           ⟨"tcp"|"udp", "tcp"⟩    M1, D1
string | *"foo"          ⟨string, "foo"⟩         M1, D1

*1 | 2 | 3               ⟨1|2|3, 1⟩              M1, D1

(*1|2|3) | (1|*2|3)      ⟨1|2|3, 1|2⟩            M1, D1, D2
(*1|2|3) | *(1|*2|3)     ⟨1|2|3, 2⟩              M1, M2, M3, D1, D2
(*1|2|3) | (1|*2|3)&2    ⟨1|2|3, 1|2⟩            M1, D1, U1, D2

(*1|2) & (1|*2)          ⟨1|2, _|_⟩              M1, D1, U2

The rules of subsumption for defaults can be derived from the above definitions
and are as follows.


Copy code
Copied!

⟨v2, d2⟩ ⊑ ⟨v1, d1⟩  if v2 ⊑ v1 and d2 ⊑ d1
⟨v1, d1⟩ ⊑ ⟨v⟩       if v1 ⊑ v
⟨v⟩      ⊑ ⟨v1, d1⟩  if v ⊑ d1


Copy code
Copied!

Expression                       Resolves to
"tcp" | "udp"                    "tcp" | "udp"
*"tcp" | "udp"                   "tcp"
float | *1                       1
*string | 1.0                    string
(*1|2) + (2|*3)                  4

(*1|2|3) | (1|*2|3)              1|2
(*1|2|3) & (1|*2|3)              1|2|3 // default is _|_

(* >=5 | int) & (* <=5 | int)    5

(*"tcp"|"udp") & ("udp"|*"tcp")  "tcp"
(*"tcp"|"udp") & ("udp"|"tcp")   "tcp"
(*"tcp"|"udp") & "tcp"           "tcp"
(*"tcp"|"udp") & (*"udp"|"tcp")  "tcp" | "udp" // default is _|_

(*true | false) & bool           true
(*true | false) & (true | false) true

{a: 1} | {b: 1}                  {a: 1} | {b: 1}
{a: 1} | *{b: 1}                 {b:1}
*{a: 1} | *{b: 1}                {a: 1} | {b: 1}
({a: 1} | {b: 1}) & {a:1}        {a:1}  | {a: 1, b: 1}
({a:1}|*{b:1}) & ({a:1}|*{b:1})  {b:1}

BOTTOM AND ERRORS

Any evaluation error in CUE results in a bottom value, represented by
the token _|_.
Bottom is an instance of every other value.
Any evaluation error is represented as bottom.

Implementations may associate error strings with different instances of bottom;
logically they all remain the same value.


Copy code
Copied!

bottom_lit = "_|_" .

TOP

Top is represented by the underscore character _, lexically an identifier.
Unifying any value v with top results in v itself.


Copy code
Copied!

Expr        Result
_ &  5        5
_ &  _        _
_ & _|_      _|_
_ | _|_       _

NULL

The null value is represented with the keyword null.
It has only one parent, top, and one child, bottom.
It is unordered with respect to any other value.


Copy code
Copied!

null_lit   = "null" .


Copy code
Copied!

null & 8     _|_
null & _     null
null & _|_   _|_

BOOLEAN VALUES

A boolean type represents the set of Boolean truth values denoted by
the keywords true and false.
The predeclared boolean type is bool; it is a defined type and a separate
element in the lattice.


Copy code
Copied!

bool_lit = "true" | "false" .


Copy code
Copied!

bool & true          true
true & true          true
true & false         _|_
bool & (false|true)  false | true
bool & (true|false)  true | false

NUMERIC VALUES

The integer type represents the set of all integral numbers.
The decimal floating-point type represents the set of all decimal floating-point
numbers.
They are two distinct types.
Both are instances instances of a generic number type.

The predeclared number, integer, and decimal floating-point types are
number, int and float; they are defined types.

A decimal floating-point literal always has type float;
it is not an instance of int even if it is an integral number.

Integer literals are always of type int and don’t match type float.

Numeric literals are exact values of arbitrary precision.
If the operation permits it, numbers should be kept in arbitrary precision.

Implementation restriction: although numeric values have arbitrary precision
in the language, implementations may implement them using an internal
representation with limited precision.
That said, every implementation must:

 * Represent integer values with at least 256 bits.
 * Represent floating-point values with a mantissa of at least 256 bits and
   a signed binary exponent of at least 16 bits.
 * Give an error if unable to represent an integer value precisely.
 * Give an error if unable to represent a floating-point value due to overflow.
 * Round to the nearest representable value if unable to represent
   a floating-point value due to limits on precision.
   These requirements apply to the result of any expression except for builtin
   functions, for which an unusual loss of precision must be explicitly documented.

STRINGS

The string type represents the set of UTF-8 strings,
not allowing surrogates.
The predeclared string type is string; it is a defined type.

The length of a string s (its size in bytes) can be discovered using
the builtin function len.

BYTES

The bytes type represents the set of byte sequences.
A byte sequence value is a (possibly empty) sequence of bytes.
The number of bytes is called the length of the byte sequence
and is never negative.
The predeclared byte sequence type is bytes; it is a defined type.

BOUNDS

A bound, syntactically a unary expression [/docs/reference/spec/#operands], defines
a logically infinite disjunction of concrete values represented as a single comparison.
For example, >= 2 represents the infinite disjunction 2|3|4|5|6|7|….

For any comparison operator [/docs/reference/spec/#comparison-operators] op,
op a is the disjunction of every x such that x op a.


Copy code
Copied!

2 & >=2 & <=5           // 2, where 2 is either an int or float.
2.5 & >=1 & <=5         // 2.5
2 & >=1.0 & <3.0        // 2.0
2 & >1 & <3.0           // 2.0
2.5 & int & >1 & <5     // _|_
2.5 & float & >1 & <5   // 2.5
int & 2 & >1.0 & <3.0   // _|_
2.5 & >=(int & 1) & <5  // _|_
>=0 & <=7 & >=3 & <=10  // >=3 & <=7
!=null & 1              // 1
==[1, 2] & [1]          // _|_
!=[1, 2] & [1]          // [1]

STRUCTS

A struct is a set of elements called fields, each of
which has a name, called a label, and value.

We say a label is defined for a struct if the struct has a field with the
corresponding label.
The value for a label f of struct a is denoted a.f.
A struct a is an instance of b, or a ⊑ b, if for any label f
defined for b, label f is also defined for a and a.f ⊑ b.f.
Note that if a is an instance of b it may have fields with labels that
are not defined for b.

The (unique) struct with no fields, written {}, has every struct as an
instance. It can be considered the type of all structs.


Copy code
Copied!

{a: 1} ⊑ {}
{a: 1, b: 1} ⊑ {a: 1}
{a: 1} ⊑ {a: int}
{a: 1, b: 1.0} ⊑ {a: int, b: number}

{} ⋢ {a: 1}
{a: 2} ⋢ {a: 1}
{a: 1} ⋢ {b: 1}

The successful unification of structs a and b is a new struct c which
has all fields of both a and b, where
the value of a field f in c is a.f & b.f if f is defined in both a and b,
or just a.f or b.f if f is in just a or b, respectively.
Any references [/docs/reference/spec/#references] to a or b
in their respective field values need to be replaced with references to c.
The result of a unification is bottom (_|_) if any of its defined
fields evaluates to bottom, recursively.

A struct literal may contain multiple fields with the same label,
the result of which is the unification of all those fields.


Copy code
Copied!

StructLit       = "{" { Declaration "," } "}" .
Declaration     = Field | Ellipsis | Embedding | LetClause | attribute .
Ellipsis        = "..." [ Expression ] .
Embedding       = Comprehension | AliasExpr .
Field           = Label ":" { Label ":" } AliasExpr { attribute } .
Label           = [ identifier "=" ] LabelExpr .
LabelExpr       = LabelName [ "?" | "!" ] | "[" AliasExpr "]" .
LabelName       = identifier | simple_string_lit | "(" AliasExpr ")" .

attribute       = "@" identifier "(" attr_tokens ")" .
attr_tokens     = { attr_token |
                    "(" attr_tokens ")" |
                    "[" attr_tokens "]" |
                    "{" attr_tokens "}" } .
attr_token      = /* any token except '(', ')', '[', ']', '{', or '}' */


Copy code
Copied!

Expression                             Result
{a: int, a: 1}                         {a: 1}
{a: int} & {a: 1}                      {a: 1}
{a: >=1 & <=7} & {a: >=5 & <=9}        {a: >=5 & <=7}
{a: >=1 & <=7, a: >=5 & <=9}           {a: >=5 & <=7}

{a: 1} & {b: 2}                        {a: 1, b: 2}
{a: 1, b: int} & {b: 2}                {a: 1, b: 2}

{a: 1} & {a: 2}                        _|_


FIELD CONSTRAINTS

A struct may declare field constraints which define values
that should be unified with a given field once it is defined.
The existence of a field constraint declares, but does not define, that field.

Syntactically, a field is marked as a constraint
by following its label with an optional marker ?
or required marker !.
These markers are not part of the field name.

A struct that has a required field constraint with a bottom value
evaluates to bottom.
An optional field constraint with a bottom value does not invalidate
the struct that contains it
as long as it is not unified with a defined field.

The subsumption relation for fields with the various markers is defined as


Copy code
Copied!

{a: x} ⊑ {a!: x} ⊑ {a?: x}

for any given x.

Implementations may error upon encountering a required field constraint
when manifesting CUE as data.


Copy code
Copied!

Expression                             Result
{foo?: 3} & {foo: 3}                   {foo: 3}
{foo!: 3} & {foo: 3}                   {foo: 3}

{foo!: int} & {foo: int}               {foo:  int}
{foo!: int} & {foo?: <1}               {foo!: <1}
{foo!: int} & {foo: <=3}               {foo:  <=3}
{foo!: int} & {foo: 3}                 {foo:  3}

{foo!: 3} & {foo: int}                 {foo: 3}
{foo!: 3} & {foo: <=4}                 {foo: 3}

{foo?: 1} & {foo?: 2}                  {foo?: _|_} // No error
{foo?: 1} & {foo!: 2}                  _|_
{foo?: 1} & {foo: 2}                   _|_


DYNAMIC FIELDS

A dynamic field is a field whose label is determined by
an expression wrapped in parentheses.
A dynamic field may be marked as optional or required.


Copy code
Copied!

Expression                             Result
a:   "foo"                             a:   "foo"
b:   "bar"                             b:   "bar"
(a): "baz"                             foo: "baz"

(a+b): "qux"                           foobar: "qux"

(a)?: string                           foo?: string
(b)!: string                           bar!: string


PATTERN AND DEFAULT CONSTRAINTS

A struct may define constraints that apply to a collection of fields.

A pattern constraint, denoted [pattern]: value, defines a pattern, which
is a value of type string, and a value to unify with fields whose label
unifies with the pattern.
For a given struct a with pattern constraint [p]: v, v is unified
with any field with name f in a for which p & f is not bottom.
When unifying struct a and b,
any pattern constraint declared in a and b
are also declared in the result of unification.

Additionally, a default constraint, denoted ...value, defines a value
to unify with any field for which there is no other declaration in a struct.
When unifying structs a and b,
a default constraint ...v declared in a
defines that the value v should unify with any field in the resulting struct c
whose label does not unify with any of the patterns of the pattern
constraints defined for a and for which there exists no field declaration
in a with that label.
The token ... is a shorthand for ..._.
Note: default constraints of the form ..._ are not yet implemented.


Copy code
Copied!

a: {
    foo:      string  // foo is a string
    [=~"^i"]: int     // all other fields starting with i are integers
    [=~"^b"]: bool    // all other fields starting with b are booleans
    [>"c"]:   string  // all other fields lexically after c are strings

    ...string         // all other fields must be a string. Note: default constraints are not yet implemented.
}

b: a & {
    i3:    3
    bar:   true
    other: "a string"
}

Concrete field labels may be an identifier or string, the latter of which may be
interpolated.
Fields with identifier labels can be referred to within the scope they are
defined, string labels cannot.
References within such interpolated strings are resolved within
the scope of the struct in which the label sequence is
defined and can reference concrete labels lexically preceding
the label within a label sequence.


Copy code
Copied!

intMap: [string]: int
intMap: {
    t1: 43
    t2: 2.4  // error: 2.4 is not an integer
}

nameMap: [string]: {
    firstName: string
    nickName:  *firstName | string
}

nameMap: hank: firstName: "Hank"

The optional field set defined by nameMap matches every field,
in this case just hank, and unifies the associated constraint
with the matched field, resulting in:


Copy code
Copied!

nameMap: hank: {
    firstName: "Hank"
    nickName:  "Hank"
}


CLOSED STRUCTS

By default, structs are open to adding fields.
Instances of an open struct p may contain fields not defined in p.
This is makes it easy to add fields, but can lead to bugs:


Copy code
Copied!

S: {
    field1: string
}

S1: S & { field2: "foo" }

// S1 is { field1: string, field2: "foo" }


A: {
    field1: string
    field2: string
}

A1: A & {
    feild1: "foo"  // "field1" was accidentally misspelled
}

// A1 is
//    { field1: string, field2: string, feild1: "foo" }
// not the intended
//    { field1: "foo", field2: string }

A closed struct c is a struct whose instances may not declare any field
with a name that does not match the name of a field
or the pattern of a pattern constraint defined in c.
Hidden fields are excluded from this limitation.
A struct that is the result of unifying any struct with a ... [/docs/reference/spec/#structs]
declaration is defined for all regular fields.
Closing a struct is equivalent to adding ..._|_ to it.

Syntactically, structs are closed explicitly with the close builtin or
implicitly and recursively by definitions [/docs/reference/spec/#definitions-and-hidden-fields].


Copy code
Copied!

A: close({
    field1: string
    field2: string
})

A1: A & {
    feild1: string
} // _|_ feild1 not defined for A

A2: A & {
    for k,v in { feild1: string } {
        k: v
    }
}  // _|_ feild1 not defined for A

C: close({
    [_]: _
})

C2: C & {
    for k,v in { thisIsFine: string } {
        "\(k)": v
    }
}

D: close({
    // Values generated by comprehensions are treated as embeddings.
    for k,v in { x: string } {
        "\(k)": v
    }
})


EMBEDDING

A struct may contain an embedded value, an operand used as a declaration.
An embedded value of type struct is unified with the struct in which it is
embedded, but disregarding the restrictions imposed by closed structs.
So if an embedding resolves to a closed struct, the corresponding enclosing
struct will also be closed, but may have fields that are not allowed if
normal rules for closed structs were observed.

If an embedded value is not of type struct, the struct may only have
definitions or hidden fields. Regular fields are not allowed in such case.

The result of { A } is A for any A (including definitions).

Syntactically, embeddings may be any expression.


Copy code
Copied!

S1: {
    a: 1
    b: 2
    {
        c: 3
    }
}
// S1 is { a: 1, b: 2, c: 3 }

S2: close({
    a: 1
    b: 2
    {
        c: 3
    }
})
// same as close(S1)

S3: {
    a: 1
    b: 2
    close({
        c: 3
    })
}
// same as S2


DEFINITIONS AND HIDDEN FIELDS

A field is a definition if its identifier starts with # or _#.
A field is hidden if its identifier starts with a _.
All other fields are regular.

Definitions and hidden fields are not emitted when converting a CUE program
to data and are never required to be concrete.

Referencing a definition will recursively close [/docs/reference/spec/#closed-structs] it.
That is, a referenced definition will not unify with a struct
that would add a field anywhere within the definition that it does not
already define or explicitly allow with a pattern constraint or ....
Embedding [/docs/reference/spec/#embedding] allows bypassing this check.

If referencing a definition would always result in an error, implementations
may report this inconsistency at the point of its declaration.


Copy code
Copied!

#MyStruct: {
    sub: field:    string
}

#MyStruct: {
    sub: enabled?: bool
}

myValue: #MyStruct & {
    sub: feild:   2     // error, feild not defined in #MyStruct
    sub: enabled: true  // okay
}

#D: {
    #OneOf

    c: int // adds this field.
}

#OneOf: { a: int } | { b: int }


D1: #D & { a: 12, c: 22 }  // { a: 12, c: 22 }
D2: #D & { a: 12, b: 33 }  // _|_ // cannot define both `a` and `b`


Copy code
Copied!

#A: {a: int}

B: {
    #A
    b: c: int
}

x: B
x: d: 3  // not allowed, as closed by embedded #A

y: B.b
y: d: 3  // allowed as nothing closes b

#B: {
    #A
    b: c: int
}

z: #B.b
z: d: 3  // not allowed, as referencing #B closes b


ATTRIBUTES

Attributes allow associating meta information with values.
Their primary purpose is to define mappings between CUE and
other representations.
Attributes do not influence the evaluation of CUE.

An attribute associates an identifier with a value, a balanced token sequence,
which is a sequence of CUE tokens with balanced brackets ((), [], and {}).
The sequence may not contain interpolations.

Fields, structs and packages can be associated with a set of attributes.
Attributes accumulate during unification, but implementations may remove
duplicates that have the same source string representation.
The interpretation of an attribute, including the handling of multiple
attributes for a given identifier, is up to the consumer of the attribute.

Field attributes define additional information about a field,
such as a mapping to a protocol buffer tag or alternative
name of the field when mapping to a different language.


Copy code
Copied!

// Package attribute
@protobuf(proto3)

myStruct1: {
    // Struct attribute:
    @jsonschema(id="https://example.org/mystruct1.json")

    // Field attributes
    field: string @go(Field)
    attr:  int    @xml(,attr) @go(Attr)
}

myStruct2: {
    field: string @go(Field)
    attr:  int    @xml(a1,attr) @go(Attr)
}

Combined: myStruct1 & myStruct2
// field: string @go(Field)
// attr:  int    @xml(,attr) @xml(a1,attr) @go(Attr)


ALIASES

Aliases name values that can be referred to
within the scope [/docs/reference/spec/#declarations-and-scopes] in which they are declared.
The name of an alias must be unique within its scope.


Copy code
Copied!

AliasExpr  = [ identifier "=" ] Expression .

Aliases can appear in several positions:

In front of a Label (X=label: value):

 * binds the identifier to the same value as label would be bound
   to if it were a valid identifier.

In front of a dynamic field (X=(label): value):

 * binds the identifier to the same value as label if it were a valid
   static identifier.

In front of a dynamic field expression ((X=expr): value):

 * binds the identifier to the concrete label resulting from evaluating expr.

In front of a pattern constraint (X=[expr]: value):

 * binds the identifier to the same field as the matched by the pattern
   within the instance of the field value (value).

In front of a pattern constraint expression ([X=expr]: value):

 * binds the identifier to the concrete label that matches expr
   within the instances of the field value (value).

Before a value (foo: X=x)

 * binds the identifier to the value it precedes within the scope of that value.

Before a list element ([ X=value, X+1 ]) (Not yet implemented)

 * binds the identifier to the list element it precedes within the scope of the
   list expression.


Copy code
Copied!

// A field alias
foo: X  // 4
X="not an identifier": 4

// A value alias
foo: X={x: X.a}
bar: foo & {a: 1}  // {a: 1, x: 1}

// A label alias
[Y=string]: { name: Y }
foo: { value: 1 } // outputs: foo: { name: "foo", value: 1 }


LET DECLARATIONS

Let declarations bind an identifier to an expression.
The identifier is only visible within the scope [/docs/reference/spec/#declarations-and-scopes]
in which it is declared.
The identifier must be unique within its scope.


Copy code
Copied!

let x = expr

a: x + 1
b: x + 2


SHORTHAND NOTATION FOR NESTED STRUCTS

A field whose value is a struct with a single field may be written as
a colon-separated sequence of the two field names,
followed by a colon and the value of that single field.


Copy code
Copied!

job: myTask: replicas: 2

expands to


Copy code
Copied!

job: {
    myTask: {
        replicas: 2
    }
}

LISTS

A list literal defines a new value of type list.
A list may be open or closed.
An open list is indicated with a ... at the end of an element list,
optionally followed by a value for the remaining elements.

The length of a closed list is the number of elements it contains.
The length of an open list is the number of elements as a lower bound
and an unlimited number of elements as its upper bound.


Copy code
Copied!

ListLit       = "[" [ ElementList [ "," ] ] "]" .
ElementList   = Ellipsis | Embedding { "," Embedding } [ "," Ellipsis ] .

Lists can be thought of as structs:


Copy code
Copied!

List: *null | {
    Elem: _
    Tail: List
}

For closed lists, Tail is null for the last element, for open lists it is
*null | List, defaulting to the shortest variant.
For instance, the open list [ 1, 2, … ] can be represented as:


Copy code
Copied!

open: List & { Elem: 1, Tail: { Elem: 2 } }

and the closed version of this list, [ 1, 2 ], as


Copy code
Copied!

closed: List & { Elem: 1, Tail: { Elem: 2, Tail: null } }

Using this representation, the subsumption rule for lists can
be derived from those of structs.
Implementations are not required to implement lists as structs.
The Elem and Tail fields are not special and len will not work as
expected in these cases.

DECLARATIONS AND SCOPES

BLOCKS

A block is a possibly empty sequence of declarations.
The braces of a struct literal { ... } form a block, but there are
others as well:

 * The universe block encompasses all CUE source text.
 * Each package [/docs/reference/spec/#modules-instances-and-packages] has a package block
   containing all CUE source text in that package.
 * Each file has a file block containing all CUE source text in that file.
 * Each for and let clause in a comprehension [/docs/reference/spec/#comprehensions]
   is considered to be its own implicit block.

Blocks nest and influence scoping.

DECLARATIONS AND SCOPE

A declaration may bind an identifier to a field, alias, or package.
Every identifier in a program must be declared.
Other than for fields,
no identifier may be declared twice within the same block.
For fields, an identifier may be declared more than once within the same block,
resulting in a field with a value that is the result of unifying the values
of all fields with the same identifier.
String labels do not bind an identifier to the respective field.

The scope of a declared identifier is the extent of source text in which the
identifier denotes the specified field, alias, or package.

CUE is lexically scoped using blocks:

 1. The scope of a predeclared identifier [/docs/reference/spec/#predeclared-identifiers] is the universe block.
 2. The scope of an identifier denoting a field
    declared at top level (outside any struct literal) is the package block.
 3. The scope of an identifier denoting an alias
    declared at top level (outside any struct literal) is the file block.
 4. The scope of a let identifier
    declared at top level (outside any struct literal) is the file block.
 5. The scope of the package name of an imported package is the file block of the
    file containing the import declaration.
 6. The scope of a field, alias or let identifier declared inside a struct
    literal is the innermost containing block.

An identifier declared in a block may be redeclared in an inner block.
While the identifier of the inner declaration is in scope, it denotes the entity
declared by the inner declaration.

The package clause is not a declaration;
the package name does not appear in any scope.
Its purpose is to identify the files belonging to the same package
and to specify the default name for import declarations.

PREDECLARED IDENTIFIERS

CUE predefines a set of types and builtin functions.
For each of these there is a corresponding keyword which is the name
of the predefined identifier, prefixed with __.


Copy code
Copied!

Functions
len close and or

Types
null      The null type and value
bool      All boolean values
int       All integral numbers
float     All decimal floating-point numbers
string    Any valid UTF-8 sequence
bytes     Any valid byte sequence

Derived   Value
number    int | float
uint      >=0
uint8     >=0 & <=255
int8      >=-128 & <=127
uint16    >=0 & <=65535
int16     >=-32_768 & <=32_767
rune      >=0 & <=0x10FFFF
uint32    >=0 & <=4_294_967_295
int32     >=-2_147_483_648 & <=2_147_483_647
uint64    >=0 & <=18_446_744_073_709_551_615
int64     >=-9_223_372_036_854_775_808 & <=9_223_372_036_854_775_807
uint128   >=0 & <=340_282_366_920_938_463_463_374_607_431_768_211_455
int128    >=-170_141_183_460_469_231_731_687_303_715_884_105_728 &
           <=170_141_183_460_469_231_731_687_303_715_884_105_727
float32   >=-3.40282346638528859811704183484516925440e+38 &
          <=3.40282346638528859811704183484516925440e+38
float64   >=-1.797693134862315708145274237317043567981e+308 &
          <=1.797693134862315708145274237317043567981e+308

EXPORTED IDENTIFIERS

An identifier of a package may be exported to permit access to it
from another package.
All identifiers not starting with _ (so all regular fields and definitions
starting with #) are exported.
Any identifier starting with _ is not visible outside the package and resides
in a separate namespace than namesake identifiers of other packages.


Copy code
Copied!

package mypackage

foo:   string  // visible outside mypackage
"bar": string  // visible outside mypackage

#Foo: {      // visible outside mypackage
    a:  1    // visible outside mypackage
    _b: 2    // not visible outside mypackage

    #C: {    // visible outside mypackage
        d: 4 // visible outside mypackage
    }
    _#E: foo // not visible outside mypackage
}

UNIQUENESS OF IDENTIFIERS

Given a set of identifiers, an identifier is called unique if it is different
from every other in the set, after applying normalization following
Unicode Annex #31 [https://unicode.org/reports/tr31/].
Two identifiers are different if they are spelled differently
or if they appear in different packages and are not exported.
Otherwise, they are the same.

FIELD DECLARATIONS

A field associates the value of an expression to a label within a struct.
If this label is an identifier, it binds the field to that identifier,
so the field’s value can be referenced by writing the identifier.
String labels are not bound to fields.


Copy code
Copied!

a: {
    b: 2
    "s": 3

    c: b   // 2
    d: s   // _|_ unresolved identifier "s"
    e: a.s // 3
}

If an expression may result in a value associated with a default value
as described in default values [/docs/reference/spec/#default-values], the field binds to this
value-default pair.

LET DECLARATIONS

Within a struct, a let clause binds an identifier to the given expression.

Within the scope of the identifier, the identifier refers to the
locally declared expression.
The expression is evaluated in the scope it was declared.

EXPRESSIONS

An expression specifies the computation of a value by applying operators and
builtin functions to operands.

Expressions that require concrete values are called incomplete if any of
their operands are not concrete, but define a value that would be legal for
that expression.
Incomplete expressions may be left unevaluated until a concrete value is
requested at the application level.

OPERANDS

Operands denote the elementary values in an expression.
An operand may be a literal, a (possibly qualified) identifier denoting
a field, alias, or let declaration, or a parenthesized expression.


Copy code
Copied!

Operand     = Literal | OperandName | "(" Expression ")" .
Literal     = BasicLit | ListLit | StructLit .
BasicLit    = int_lit | float_lit | string_lit |
              null_lit | bool_lit | bottom_lit .
OperandName = identifier | QualifiedIdent .

QUALIFIED IDENTIFIERS

A qualified identifier is an identifier qualified with a package name prefix.


Copy code
Copied!

QualifiedIdent = PackageName "." identifier .

A qualified identifier accesses an identifier in a different package,
which must be imported [/docs/reference/spec/#import-declarations].
The identifier must be declared in the package block [/docs/reference/spec/#blocks] of that package.


Copy code
Copied!

math.Sin    // denotes the Sin function in package math

REFERENCES

An identifier operand refers to a field and is called a reference.
The value of a reference is a copy of the expression associated with the field
that it is bound to,
with any references within that expression bound to the respective copies of
the fields they were originally bound to.
Implementations may use a different mechanism to evaluate as long as
these semantics are maintained.


Copy code
Copied!

a: {
    place:    string
    greeting: "Hello, \(place)!"
}

b: a & { place: "world" }
c: a & { place: "you" }

d: b.greeting  // "Hello, world!"
e: c.greeting  // "Hello, you!"

PRIMARY EXPRESSIONS

Primary expressions are the operands for unary and binary expressions.


Copy code
Copied!

PrimaryExpr =
	Operand |
	PrimaryExpr Selector |
	PrimaryExpr Index |
	PrimaryExpr Arguments .

Selector       = "." (identifier | simple_string_lit) .
Index          = "[" Expression "]" .
Argument       = Expression .
Arguments      = "(" [ ( Argument { "," Argument } ) [ "," ] ] ")" .


Copy code
Copied!

x
2
(s + ".txt")
f(3.1415, true)
m["foo"]
obj.color
f.p[i].x

SELECTORS

For a primary expression [/docs/reference/spec/#primary-expressions] x that is not a package name [/docs/reference/spec/#package-clause],
the selector expression


Copy code
Copied!

x.f

denotes the element of a struct x identified by f.

f must be an identifier or a string literal identifying
any definition or regular non-optional field.
The identifier f is called the field selector.

If x is a package name, see the section on qualified identifiers [/docs/reference/spec/#qualified-identifiers].

Otherwise, if x is not a struct,
or if f does not exist in x,
the result of the expression is bottom (an error).
In the latter case the expression is incomplete.
The operand of a selector may be associated with a default.


Copy code
Copied!

T: {
    x:     int
    y:     3
    "x-y": 4
}

a: T.x     // int
b: T.y     // 3
c: T.z     // _|_ // field 'z' not found in T
d: T."x-y" // 4

e: {a: 1|*2} | *{a: 3|*4}
f: e.a  // 4 (default value)

INDEX EXPRESSIONS

A primary expression of the form


Copy code
Copied!

a[x]

denotes the element of a list or struct a indexed by x.
The value x is called the index or field name, respectively.
The following rules apply:

If a is not a struct:

 * a is a list (which need not be complete)
 * the index x unified with int must be concrete.
 * the index x is in range if 0 <= x < len(a), where only the
   explicitly defined values of an open-ended list are considered,
   otherwise it is out of range

The result of a[x] is

for a of list type:

 * the list element at index x, if x is within range
 * bottom (an error), otherwise

for a of struct type:

 * the index x unified with string must be concrete.
 * the value of the regular and non-optional field named x of struct a,
   if this field exists
 * bottom (an error), otherwise


Copy code
Copied!

a: [ 1, 2 ][1]     // 2
b: [ 1, 2 ][2]     // _|_
c: [ 1, 2, ...][2] // _|_

// Defaults are selected for both operand and index:
x: [1, 2] | *[3, 4]
y: int | *1
z: x[y]  // 4

OPERATORS

Operators combine operands into expressions.


Copy code
Copied!

Expression = UnaryExpr | Expression binary_op Expression .
UnaryExpr  = PrimaryExpr | unary_op UnaryExpr .

binary_op  = "|" | "&" | "||" | "&&" | "==" | rel_op | add_op | mul_op  .
rel_op     = "!=" | "<" | "<=" | ">" | ">=" | "=~" | "!~" .
add_op     = "+" | "-" .
mul_op     = "*" | "/" .
unary_op   = "+" | "-" | "!" | "*" | rel_op .

Comparisons are discussed elsewhere [/docs/reference/spec/#comparison-operators].
For any binary operators, the operand types must unify.


OPERATOR PRECEDENCE

Unary operators have the highest precedence.

There are eight precedence levels for binary operators.
Multiplication operators binds strongest, followed by
addition operators, comparison operators,
&& (logical AND), || (logical OR), & (unification),
and finally | (disjunction):


Copy code
Copied!

Precedence    Operator
    7             *  /
    6             +  -
    5             ==  !=  <  <=  >  >= =~ !~
    4             &&
    3             ||
    2             &
    1             |

Binary operators of the same precedence associate from left to right.
For instance, x / y * z is the same as (x / y) * z.


Copy code
Copied!

+x
23 + 3*x[i]
x <= f()
f() || g()
x == y+1 && y == z-1
2 | int
{ a: 1 } & { b: 2 }


ARITHMETIC OPERATORS

Arithmetic operators apply to numeric values and yield a result of the same type
as the first operand. The four standard arithmetic operators
(+, -, *, /) apply to integer and decimal floating-point types;
+ and * also apply to strings and bytes.


Copy code
Copied!

+    sum                    integers, floats, strings, bytes
-    difference             integers, floats
*    product                integers, floats, strings, bytes
/    quotient               integers, floats

For any operator that accepts operands of type float, any operand may be
of type int or float, in which case the result will be float
if it cannot be represented as an int or if any of the operands are float,
or int otherwise.
So the result of 1 / 2 is 0.5 and is of type float.

The result of division by zero is bottom (an error).

Integer division is implemented through the builtin functions
quo, rem, div, and mod.

The unary operators + and - are defined for numeric values as follows:


Copy code
Copied!

+x                          is 0 + x
-x    negation              is 0 - x


STRING OPERATORS

Strings can be concatenated using the + operator:


Copy code
Copied!

s: "hi " + name + " and good bye"

String addition creates a new string by concatenating the operands.

A string can be repeated by multiplying it:


Copy code
Copied!

s: "etc. "*3  // "etc. etc. etc. "


COMPARISON OPERATORS

Comparison operators compare two concrete operands and yield a boolean value.


Copy code
Copied!

==    equal
!=    not equal
<     less
<=    less or equal
>     greater
>=    greater or equal
=~    matches regular expression
!~    does not match regular expression

In any comparison, both operands must be concrete; otherwise the result is
bottom (_|_).

The equality operators == and != can be applied to any two concrete
operands.
The ordering operators <, <=, >, and >= apply only to operands of the
same ordered type (numeric, string, or bytes).
The matching operators =~ and !~ apply to a string and a regular expression
operand.

For equality comparisons (== and !=):

 * Two values of different basic types are always unequal, except for integers
   and floating-point numbers (see below).
 * Null values are equal only to other null values.
 * Boolean values are equal if they are both true or both false.
 * Numeric values are equal if they represent the same number.
   When comparing an integer with a floating-point number, the integer is first
   converted to floating-point.
 * String values are equal if they contain the same sequence of bytes.
 * Bytes values are equal if they contain the same sequence of bytes.
 * Struct values are equal if they have the same set of regular field labels
   and the corresponding values are recursively equal. Only regular fields are
   considered; field order and closedness are irrelevant.
 * List values are equal if they have the same length and their corresponding
   elements are recursively equal.

For ordering comparisons (<, <=, >, >=):

 * Numeric values are ordered by their numeric value, with integer-to-float
   conversion as described above.
 * String values are ordered lexically byte-wise.
 * Bytes values are ordered lexically byte-wise.

For pattern matching (=~, !~):

 * The regular expression syntax is that accepted by RE2 (https://github.com/google/re2/wiki/Syntax) [https://github.com/google/re2/wiki/Syntax%29], except for \C.
 * s =~ r is true if string s matches regular expression r.
 * s !~ r is true if string s does not match regular expression r.


Copy code
Copied!

3 < 4       // true
3 < 4.0     // true
null == 2   // false
null != {}  // true
{} == {}    // _|_: structs are not comparable against structs

"Wild cats" =~ "cat"   // true
"Wild cats" !~ "dog"   // true

"foo" =~ "^[a-z]{3}$"  // true
"foo" =~ "^[a-z]{4}$"  // false


LOGICAL OPERATORS

Logical operators apply to boolean values and yield a result of the same type
as the operands. The right operand is evaluated conditionally.


Copy code
Copied!

&&    conditional AND    p && q  is  "if p then q else false"
||    conditional OR     p || q  is  "if p then true else q"
!     NOT                !p      is  "not p"

CALLS

Calls can be made to core library functions, called builtins.
Given an expression f of function type F,


Copy code
Copied!

f(a1, a2, … an)

calls f with arguments a1, a2, … an. Arguments must be expressions
of which the values are an instance of the parameter types of F
and are evaluated before the function is called.


Copy code
Copied!

a: math.Atan2(x, y)

In a function call, the function value and arguments are evaluated in the usual
order.
After they are evaluated, the parameters of the call are passed by value
to the function and the called function begins execution.
The return parameters
of the function are passed by value back to the calling function when the
function returns.

COMPREHENSIONS

Lists and fields can be constructed using comprehensions.

Comprehensions define a clause sequence that consists of a sequence of
for, if, and let clauses, nesting from left to right.
The sequence must start with a for or if clause.
The for and let clauses each define a new scope in which new values are
bound to be available for the next clause.

The for clause binds the defined identifiers, on each iteration, to the next
value of some iterable value in a new scope.
A for clause may bind one or two identifiers.
If there is one identifier, it binds it to the value of
a list element or struct field value.
If there are two identifiers, the first value will be the key or index,
if available, and the second will be the value.

For lists, for iterates over all elements in the list after closing it.
For structs, for iterates over all non-optional regular fields.

An if clause, or guard, specifies an expression that terminates the current
iteration if it evaluates to false.

The let clause binds the result of an expression to the defined identifier
in a new scope.

A current iteration is said to complete if the innermost block of the clause
sequence is reached.
Syntactically, the comprehension value is a struct.
A comprehension can generate non-struct values by embedding such values within
this struct.

Within lists, the values yielded by a comprehension are inserted in the list
at the position of the comprehension.
Within structs, the values yielded by a comprehension are embedded within the
struct.
Both structs and lists may contain multiple comprehensions.


Copy code
Copied!

Comprehension       = Clauses StructLit .

Clauses             = StartClause { [ "," ] Clause } .
StartClause         = ForClause | GuardClause .
Clause              = StartClause | LetClause .
ForClause           = "for" identifier [ "," identifier ] "in" Expression .
GuardClause         = "if" Expression .
LetClause           = "let" identifier "=" Expression .


Copy code
Copied!

a: [1, 2, 3, 4]
b: [for x in a if x > 1 { x+1 }]  // [3, 4, 5]

c: {
    for x in a
    if x < 4
    let y = 1 {
        "\(x)": x + y
    }
}
d: { "1": 2, "2": 3, "3": 4 }

STRING INTERPOLATION

String interpolation allows constructing strings by replacing placeholder
expressions with their string representation.
String interpolation may be used in single- and double-quoted strings, as well
as their multiline equivalent.

A placeholder consists of \( followed by an expression and ).
The expression is evaluated in the scope within which the string is defined.

The result of the expression is substituted as follows:

 * string: as is
 * bool: the JSON representation of the bool
 * number: a JSON representation of the number that preserves the
   precision of the underlying binary-coded decimal
 * bytes: as if substituted within single quotes or
   converted to valid UTF-8 replacing the
   maximal subpart of ill-formed subsequences with a single
   replacement character (W3C encoding standard) otherwise
 * list: illegal
 * struct: illegal


Copy code
Copied!

a: "World"
b: "Hello \( a )!" // Hello World!

BUILTIN FUNCTIONS

Builtin functions are predeclared. They are called like any other function.

ERROR

The error builtin allows users to create custom error values with a specified
message.
User-generated errors can be included in disjunctions; if at least one disjunct
is valid, any user errors are ignored.
However, if all disjuncts fail, all user error messages are reported together.

error takes a single string argument. If this argument is a literal
interpolation, it will be extra resilient: if any of the arguments to the
interpolation fail, they will be printed as an expression. This allows failing
expressions to be a part of the error message.


Copy code
Copied!

a: 1/0 | error("infinity and beyond!: \(1/0)")

LEN

The builtin function len takes arguments of various types and returns
a result of type int.


Copy code
Copied!

Argument type    Result

bytes            length of byte sequence
list             list length, smallest length for an open list
struct           number of distinct data fields, excluding field constraints


Copy code
Copied!

Expression           Result
len("Hellø")         6
len([1, 2, 3])       3
len([1, 2, ...])     2

CLOSE

The builtin function close converts a partially defined, or open, struct
to a fully defined, or closed, struct.

AND

The builtin function and takes a list and returns the result of applying
the & operator to all elements in the list.
It returns top for the empty list.


Copy code
Copied!

Expression:          Result
and([a, b])          a & b
and([a])             a
and([])              _

OR

The builtin function or takes a list and returns the result of applying
the | operator to all elements in the list.
It returns bottom for the empty list.


Copy code
Copied!

Expression:          Result
or([a, b])           a | b
or([a])              a
or([])               _|_

DIV, MOD, QUO AND REM

For two integer values x and y,
the integer quotient q = div(x, y) and remainder r = mod(x, y)
implement Euclidean division and
satisfy the following relationship:


Copy code
Copied!

r = x - y*q  with 0 <= r < |y|

where |y| denotes the absolute value of y.


Copy code
Copied!

 x     y   div(x, y)  mod(x, y)
 5     3        1          2
-5     3       -2          1
 5    -3       -1          2
-5    -3        2          1

For two integer values x and y,
the integer quotient q = quo(x, y) and remainder r = rem(x, y)
implement truncated division and
satisfy the following relationship:


Copy code
Copied!

x = q*y + r  and  |r| < |y|

with quo(x, y) truncated towards zero.


Copy code
Copied!

 x     y   quo(x, y)  rem(x, y)
 5     3        1          2
-5     3       -1         -2
 5    -3       -1          2
-5    -3        1         -2

A zero divisor in either case results in bottom (an error).

BUILTIN VALIDATORS

A validator validates the value at the position where it is defined.
A successful validation yields the original value;
a failed validation yields an error.

Bounds (<10) are a type of validator.

Functions that return a boolean value can be used as validators by omitting
their first argument.

The remainder of this section defines builtin validators. These can only be
used as validators, so we will not refer to their function equivalents.

These builtins refer to finalized values, which means that the value being
validated is fully resolved, and defaults taken, before it is unified with the
schema.

MATCHN

The matchN builtin is a validator that checks if a specified number of schemas
from a given list unify successfully with the finalized value being validated.

matchN takes two arguments:

 * a numeric constraint specifying how many schemas must match,
 * a list of schemas to test against the value.

The validator evaluates each schema in the list by unifying it with the value.
It counts how many schemas unify successfully (without producing an error).
The validator succeeds if the count satisfies the numeric constraint provided
as the first argument.


Copy code
Copied!

// Exactly 2 schemas must match
value: "foo" & matchN(2, [string, !="bar", <4])  // true: string and !="bar" match

// At least 1 schema must match
value: 5 & matchN(>=1, [int, >10])  // true: int matches

// Exactly 0 schemas must match (none should match)
value: "test" & matchN(0, [int, >100])  // true: neither matches

If the numeric constraint cannot be satisfied even with incomplete information,
the error is marked as incomplete and will be reevaluated as more information
becomes available.

MATCHIF

The matchIf builtin is a conditional validator that applies different schema
constraints based on whether an initial condition is satisfied.

matchIf takes three arguments:

 * a condition schema (the “if” clause),
 * the schema to apply if the condition matches (the “then” clause),
 * the schema to apply if the condition does not match (the “else” clause).

The validator first attempts to unify the finalized value with the condition
schema.
If the condition unifies successfully, the “then” schema is applied;
otherwise, the “else” schema is applied.
The validator succeeds if the chosen schema unifies successfully with the value.


Copy code
Copied!

// If value is a string, it must have length > 3; otherwise it must be > 10
value: "hello" & matchIf(string, len(value) > 3, value > 10)  // true

// If value matches {a: int}, it must have b field; otherwise a must be a string
x: {a: 1} & matchIf(x, {a: int}, {a: int, b: int}, {a: string})  // false: missing b

// If value is >5, it must be <10; otherwise it must be <3
y: 2 & matchIf(y, >5, <10, <3)  // true: 2 is <=5, so <3 is checked

CYCLES

Implementations are required to interpret or reject cycles encountered
during evaluation according to the rules in this section.

REFERENCE CYCLES

A reference cycle occurs if a field references itself, either directly or
indirectly.


Copy code
Copied!

// x references itself
x: x

// indirect cycles
b: c
c: d
d: b

Implementations should treat these as _.
Two particular cases are discussed below.


EXPRESSIONS THAT UNIFY AN ATOM WITH AN EXPRESSION

An expression of the form a & e, where a is an atom
and e is an expression, always evaluates to a or bottom.
As it does not matter how we fail, we can assume the result to be a
and postpone validating a == e until after all references
in e have been resolved.


Copy code
Copied!

// Config            Evaluates to (requiring concrete values)
x: {                  x: {
    a: b + 100            a: _|_ // cycle detected
    b: a - 100            b: _|_ // cycle detected
}                     }

y: x & {              y: {
    a: 200                a: 200 // asserted that 200 == b + 100
                          b: 100
}                     }


FIELD VALUES

A field value of the form r & v,
where r evaluates to a reference cycle and v is a concrete value,
evaluates to v.
Unification is idempotent and unifying a value with itself ad infinitum,
which is what the cycle represents, results in this value.
Implementations should detect cycles of this kind, ignore r,
and take v as the result of unification.


Copy code
Copied!

Configuration    Evaluated
//    c           Cycles in nodes of type struct evaluate
//  ↙︎   ↖         to the fixed point of unifying their
// a  →  b        values ad infinitum.

a: b & { x: 1 }   // a: { x: 1, y: 2, z: 3 }
b: c & { y: 2 }   // b: { x: 1, y: 2, z: 3 }
c: a & { z: 3 }   // c: { x: 1, y: 2, z: 3 }

// resolve a             b & {x:1}
// substitute b          c & {y:2} & {x:1}
// substitute c          a & {z:3} & {y:2} & {x:1}
// eliminate a (cycle)   {z:3} & {y:2} & {x:1}
// simplify              {x:1,y:2,z:3}

This rule also applies to field values that are disjunctions of unification
operations of the above form.


Copy code
Copied!

a: b&{x:1} | {y:1}  // {x:1,y:3,z:2} | {y:1}
b: {x:2} | c&{z:2}  // {x:2} | {x:1,y:3,z:2}
c: a&{y:3} | {z:3}  // {x:1,y:3,z:2} | {z:3}


// resolving a           b&{x:1} | {y:1}
// substitute b          ({x:2} | c&{z:2})&{x:1} | {y:1}
// simplify              c&{z:2}&{x:1} | {y:1}
// substitute c          (a&{y:3} | {z:3})&{z:2}&{x:1} | {y:1}
// simplify              a&{y:3}&{z:2}&{x:1} | {y:1}
// eliminate a (cycle)   {y:3}&{z:2}&{x:1} | {y:1}
// expand                {x:1,y:3,z:2} | {y:1}

Note that all nodes that form a reference cycle to form a struct will evaluate
to the same value.
If a field value is a disjunction, any element that is part of a cycle will
evaluate to this value.

STRUCTURAL CYCLES

A structural cycle is when a node references one of its ancestor nodes.
It is possible to construct a structural cycle by unifying two acyclic values:


Copy code
Copied!

// acyclic
y: {
    f: h: g
    g: _
}
// acyclic
x: {
    f: _
    g: f
}
// introduces structural cycle
z: x & y

Implementations should be able to detect such structural cycles dynamically.

A structural cycle can result in infinite structure or evaluation loops.


Copy code
Copied!

// infinite structure
a: b: a

// infinite evaluation
f: {
    n:   int
    out: n + (f & {n: 1}).out
}

CUE must allow or disallow structural cycles under certain circumstances.

If a node a references an ancestor node, we call it and any of its
field values a.f cyclic.
So if a is cyclic, all of its descendants are also regarded as cyclic.
A given node x, whose value is composed of the conjuncts c1 & ... & cn,
is valid if any of its conjuncts is not cyclic.


Copy code
Copied!

// Disallowed: a list of infinite length with all elements being 1.
#List: {
    head: 1
    tail: #List
}

// Disallowed: another infinite structure (a:{b:{d:{b:{d:{...}}}}}, ...).
a: {
    b: c
}
c: {
    d: a
}

// #List defines a list of arbitrary length. Because the recursive reference
// is part of a disjunction, this does not result in a structural cycle.
#List: {
    head: _
    tail: null | #List
}

// Usage of #List. The value of tail in the most deeply nested element will
// be `null`: as the value of the disjunct referring to list is the only
// conjunct, all conjuncts are cyclic and the value is invalid and so
// eliminated from the disjunction.
MyList: #List & { head: 1, tail: { head: 2 }}

MODULES, INSTANCES, AND PACKAGES

CUE configurations are constructed combining instances.
An instance, in turn, is constructed from one or more source files belonging
to the same package that together declare the data representation.
Elements of this data representation may be exported and used
in other instances.

SOURCE FILE ORGANIZATION

Each source file consists of an optional package clause defining collection
of files to which it belongs,
followed by a possibly empty set of import declarations that declare
packages whose contents it wishes to use, followed by a possibly empty set of
declarations.

Like with a struct, a source file may contain embeddings.
Unlike with a struct, the embedded expressions may be any value.
If the result of the unification of all embedded values is not a struct,
it will be output instead of its enclosing file when exporting CUE
to a data format


Copy code
Copied!

SourceFile = { attribute "," } [ PackageClause "," ] { ImportDecl "," } { Declaration "," } .


Copy code
Copied!

"Hello \(#place)!"

#place: "world"

// Outputs "Hello world!"

PACKAGE CLAUSE

A package clause is an optional clause that defines the package to which
a source file the file belongs.


Copy code
Copied!

PackageClause  = "package" PackageName .
PackageName    = identifier .

The PackageName must not be a definition identifier.

If the PackageName is the blank identifier (_), it is treated the same
as if there were no package clause. This can be useful to allow adding
package level attributes or doc comments to a CUE file without a package
name.


Copy code
Copied!

package math

MODULES AND INSTANCES

A module defines a tree of directories, rooted at the module root.

All source files within a module with the same package name belong to the same
package.

A module may define multiple packages.

An instance of a package is any subset of files belonging
to the same package.

It is interpreted as the concatenation of these files.

An implementation may impose conventions on the layout of package files
to determine which files of a package belongs to an instance.
For example, an instance may be defined as the subset of package files
belonging to a directory and all its ancestors.

IMPORT DECLARATIONS

An import declaration states that the source file containing the declaration
depends on definitions of the imported package
and enables access to exported identifiers of that package.
The import names an identifier (PackageName) to be used for access and an
ImportPath that specifies the package to be imported.


Copy code
Copied!

ImportDecl       = "import" ( ImportSpec | "(" { ImportSpec "," } ")" ) .
ImportSpec       = [ PackageName ] ImportPath .
ImportLocation   = { unicode_value } .
ImportPath       = `"` ImportLocation [ ":" identifier ] `"` .

The PackageName is used in qualified identifiers to access
exported identifiers of the package within the importing source file.
It is declared in the file block.
It defaults to the identifier specified in the package clause of the imported
package, which must match either the last path component of ImportLocation
or the identifier following it.

The interpretation of the ImportPath is implementation-dependent but it is
typically either the path of a builtin package or a fully qualifying location
of a package within a source code repository.

An ImportLocation must be a non-empty string using only characters belonging to
Unicode’s L, M, N, P, and S general categories
(the Graphic characters without spaces)
and may not include the characters !"#$%&'()*,:;<=>?[\\]^`{|}
or the Unicode replacement character U+FFFD.

Assume we have package containing the package clause package math,
which exports function Sin at the path identified by lib/math.
This table illustrates how Sin is accessed in files
that import the package after the various types of import declaration.


Copy code
Copied!

Import declaration          Local name of Sin

import   "lib/math"         math.Sin
import   "lib/math:math"    math.Sin
import m "lib/math"         m.Sin

An import declaration declares a dependency relation between the importing and
imported package. It is illegal for a package to import itself, directly or
indirectly, or to directly import a package without referring to any of its
exported identifiers.

AN EXAMPLE PACKAGE

TODO

Last modified November 6, 2025 [https://github.com/cue-lang/cuelang.org/commit/c4d2e727a59f5158a2be4946992d96c4612a9b88]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/reference/spec/&text=%20Note%20to%20implementors%20Notes%20on%20the%20formalism%20underlying%20this%20specification%20can%20be%20found%20here.%0aIntroduction%20This%20is%20a%20reference%20manual%20for%20the%20CUE%20data%20constraint%20language.%20CUE,%20pronounced%20cue%20or%20Q,%20is%20a%20general-purpose%20and%20strongly%20typed%20constraint-based%20language.%20It%20can%20be%20used%20for%20data%20templating,%20data%20validation,%20code%20generation,%20scripting,%20and%20many%20other%20applications%20involving%20structured%20data.%20The%20CUE%20tooling,%20layered%20on%20top%20of%20CUE,%20provides%20a%20general%20purpose%20scripting%20language%20for%20creating%20scripts%20as%20well%20as%20simple%20servers,%20also%20expressed%20in%20CUE.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/reference/spec/&summary=%20Note%20to%20implementors%20Notes%20on%20the%20formalism%20underlying%20this%20specification%20can%20be%20found%20here.%0aIntroduction%20This%20is%20a%20reference%20manual%20for%20the%20CUE%20data%20constraint%20language.%20CUE,%20pronounced%20cue%20or%20Q,%20is%20a%20general-purpose%20and%20strongly%20typed%20constraint-based%20language.%20It%20can%20be%20used%20for%20data%20templating,%20data%20validation,%20code%20generation,%20scripting,%20and%20many%20other%20applications%20involving%20structured%20data.%20The%20CUE%20tooling,%20layered%20on%20top%20of%20CUE,%20provides%20a%20general%20purpose%20scripting%20language%20for%20creating%20scripts%20as%20well%20as%20simple%20servers,%20also%20expressed%20in%20CUE.%0a]


Contribution Guide
[/docs/reference/contribution-guidelines/]Glossary of terms
[/docs/reference/glossary/]
 * Introduction [/docs/introduction/]
 * Tour [/docs/tour/]
 * Integrations [/docs/integration/]
 * Tutorials [/docs/tutorial/]
 * How-to Guides [/docs/howto/]
 * Concept Guides [/docs/concept/]
 * References [/docs/reference/]
   * The CUE Language Specification [/docs/reference/spec/]
      1.  Introduction
      2.  Notation
      3.  Source code representation
      4.  Lexical elements
      5.  Values
      6.  Declarations and Scopes
      7.  Expressions
      8.  Builtin Functions
      9.  Builtin Validators
      10. Cycles
      11. Modules, instances, and packages
   
   * Glossary of terms [/docs/reference/glossary/]
   * The cue command [/docs/reference/command/]
   * Code of Conduct [/docs/reference/code-of-conduct/]

Hide side navigation


Show side navigation

Get Started

 * Documentation [/docs/]
 * Language Tour [/docs/tour/]
 * Playground [/play/]
 * Install CUE [/docs/introduction/installation/]

Community

 * The CUE Community [/community]
 * Contributing [https://github.com/cue-lang/cue/blob/master/CONTRIBUTING.md#contribution-guide]
 * Code of Conduct [/docs/reference/code-of-conduct/]
 * Slack Workspace [/s/slack]
 * Discord Server [/s/discord]

Connect

 * GitHub [https://github.com/cue-lang/cue]
 * X (Twitter) [https://twitter.com/cue_lang]
 * Bluesky [https://bsky.app/profile/cuelang.org]
 * YouTube [https://www.youtube.com/@cuelang/videos]

 * © 2025 CUE
 * Privacy policy [/privacy-policy/]
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Freference%2Fspec%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


Homepage of CUE [/]
CUE v0.15 is now available – learn more about its new features and improvements [https://github.com/cue-lang/cue/releases/tag/v0.15.0]
Install CUE

[/docs/introduction/installation/]

Close

Homepage of CUE [/]


Hide menu
 * Documentation [/docs/]
 * Play [/play/]
 * Community [/community/]
 * Install [/docs/introduction/installation/]
 * 

 * 
   GitHub [https://github.com/cue-lang/cue]
 * 
   Slack [/s/slack]
 * 
   Discord [/s/discord]
 * 
   X (Twitter) [https://twitter.com/cue_lang]
 * 
   Bluesky [https://bsky.app/profile/cuelang.org]
 * 
   YouTube [https://www.youtube.com/@cuelang/videos]
## See Also

- [Documentation Index](./COMPASS.md)
