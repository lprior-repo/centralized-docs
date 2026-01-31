---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#1-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1025
summary: Implementation restriction: For compatibility with other tools, a compiler may. ignore a UTF-8-encoded byte order mark (U+FEFF) if it is the first Unicode code
---


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
