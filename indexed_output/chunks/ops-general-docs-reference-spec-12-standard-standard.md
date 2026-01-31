---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#12-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 515
summary: and are as follows. ⟨v2, d2⟩ ⊑ ⟨v1, d1⟩  if v2 ⊑ v1 and d2 ⊑ d1
---

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
