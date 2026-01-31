---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#8-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1032
summary: Syntactically, a field is marked as a constraint. by following its label with an optional marker ?
---


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
