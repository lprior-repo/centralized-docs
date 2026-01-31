---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#25-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 512
summary: An expression specifies the computation of a value by applying operators and. builtin functions to operands
---


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
