---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#31-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 528
summary: ForClause           = \"for\" identifier [ \",\" identifier ] \"in\" Expression . GuardClause         = \"if\" Expression 
---

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
