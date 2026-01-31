---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#16-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 513
summary: evaluates to bottom. An optional field constraint with a bottom value does not invalidate
---

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
