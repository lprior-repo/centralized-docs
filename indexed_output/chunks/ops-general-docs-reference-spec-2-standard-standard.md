---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#2-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 522
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
