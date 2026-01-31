---
doc_id: ops/general/docs-tour-expressions
chunk_id: ops/general/docs-tour-expressions#0-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 527
summary: # Operators | CUE. **Source:** https://cuelang
---

# Operators | CUE

**Source:** https://cuelang.org/docs/tour/expressions/

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

 1. Tour [https://cuelang.org/docs/tour/]
 2. Expressions [https://cuelang.org/docs/tour/expressions/]


 3. OPERATORS

CUE supports common arithmetic and boolean operators, which apply to its
numerical types int, float, and number.
Some of these operators also apply to the string and bytes types.

CUE provides three different division operations.
To produce a float, CUE supports the infix / operator with the standard
mathematical meaning.
Arguments can be float, int, or a mixture of the two.
CUE also provides
built-in functions [/docs/howto/use-the-built-in-functions-div-mod-quo-rem/]
that produce an int by calculating integer division and remainder.
They support Euclidean division (div / mod) and truncated division (quo / rem).

Copied!
operators.cue

Copy code
Copied!

a: 1 + 1             // type int
b: "xxx" + "OOO"     // type string
c: 2 * 2             // type int
d: 2 * 2.0           // type float
e: 3.14159 - 0.14159 // type float

f: 6 / 2     // type: float
g: 6.0 / 2.0 // type: float

h: 1 <= 2.0 // type bool
i: 3.0 > 3  // type bool
// String types are compared lexically byte-wise
j: "aaa" >= "ZZZ" // type bool

k: div(10, 3) // type int
l: mod(10, 3) // type int

TERMINAL

Copy code
Copied!

$ cue eval operators.cue
a: 2
b: "xxxOOO"
c: 4
d: 4.0
e: 3.00000
f: 3.0
g: 3.0
h: true
i: false
j: true
k: 3
l: 1

The CUE languge specification [/docs/reference/spec/#operators]
details the operator precedence levels.

Last modified September 23, 2025 [https://github.com/cue-lang/cuelang.org/commit/af004fcc0845b84296228c157951aba972957888]
