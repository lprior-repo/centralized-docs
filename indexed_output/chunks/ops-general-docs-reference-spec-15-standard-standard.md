---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#15-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 517
summary: f if f is in just a or b, respectively. Any references [/docs/reference/spec/#references] to a or b
---

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
