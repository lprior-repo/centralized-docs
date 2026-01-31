---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#15-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1037
summary: Given an expression f of function type F,. f(a1, a2, … an)
---

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
