---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#13-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1029
summary: these semantics are maintained.     place:    string
---

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
Copied!

PrimaryExpr =
	Operand |
	PrimaryExpr Selector |
	PrimaryExpr Index |
	PrimaryExpr Arguments .

Selector       = "." (identifier | simple_string_lit) .
Index          = "[" Expression "]" .
Argument       = Expression .
Arguments      = "(" [ ( Argument { "," Argument } ) [ "," ] ] ")" .


Copy code
Copied!

x
2
(s + ".txt")
f(3.1415, true)
m["foo"]
obj.color
f.p[i].x

SELECTORS

For a primary expression [/docs/reference/spec/#primary-expressions] x that is not a package name [/docs/reference/spec/#package-clause],
the selector expression


Copy code
Copied!

x.f

denotes the element of a struct x identified by f.

f must be an identifier or a string literal identifying
any definition or regular non-optional field.
The identifier f is called the field selector.

If x is a package name, see the section on qualified identifiers [/docs/reference/spec/#qualified-identifiers].

Otherwise, if x is not a struct,
or if f does not exist in x,
the result of the expression is bottom (an error).
In the latter case the expression is incomplete.
The operand of a selector may be associated with a default.


Copy code
Copied!

T: {
    x:     int
    y:     3
    "x-y": 4
}

a: T.x     // int
b: T.y     // 3
c: T.z     // _|_ // field 'z' not found in T
d: T."x-y" // 4

e: {a: 1|*2} | *{a: 3|*4}
f: e.a  // 4 (default value)

INDEX EXPRESSIONS

A primary expression of the form


Copy code
Copied!

a[x]

denotes the element of a list or struct a indexed by x.
The value x is called the index or field name, respectively.
The following rules apply:

If a is not a struct:

 * a is a list (which need not be complete)
 * the index x unified with int must be concrete.
 * the index x is in range if 0 <= x < len(a), where only the
   explicitly defined values of an open-ended list are considered,
   otherwise it is out of range

The result of a[x] is

for a of list type:

 * the list element at index x, if x is within range
 * bottom (an error), otherwise

for a of struct type:

 * the index x unified with string must be concrete.
 * the value of the regular and non-optional field named x of struct a,
   if this field exists
 * bottom (an error), otherwise


Copy code
Copied!

a: [ 1, 2 ][1]     // 2
b: [ 1, 2 ][2]     // _|_
c: [ 1, 2, ...][2] // _|_

// Defaults are selected for both operand and index:
x: [1, 2] | *[3, 4]
y: int | *1
z: x[y]  // 4

OPERATORS

Operators combine operands into expressions.


Copy code
Copied!

Expression = UnaryExpr | Expression binary_op Expression .
UnaryExpr  = PrimaryExpr | unary_op UnaryExpr .

binary_op  = "|" | "&" | "||" | "&&" | "==" | rel_op | add_op | mul_op  .
rel_op     = "!=" | "<" | "<=" | ">" | ">=" | "=~" | "!~" .
add_op     = "+" | "-" .
mul_op     = "*" | "/" .
unary_op   = "+" | "-" | "!" | "*" | rel_op .

Comparisons are discussed elsewhere [/docs/reference/spec/#comparison-operators].
For any binary operators, the operand types must unify.


OPERATOR PRECEDENCE

Unary operators have the highest precedence.

There are eight precedence levels for binary operators.
Multiplication operators binds strongest, followed by
addition operators, comparison operators,
&& (logical AND), || (logical OR), & (unification),
and finally | (disjunction):


Copy code
Copied!

Precedence    Operator
    7             *  /
    6             +  -
    5             ==  !=  <  <=  >  >= =~ !~
    4             &&
    3             ||
    2             &
    1             |

Binary operators of the same precedence associate from left to right.
For instance, x / y * z is the same as (x / y) * z.


Copy code
Copied!

+x
23 + 3*x[i]
x <= f()
f() || g()
x == y+1 && y == z-1
2 | int
{ a: 1 } & { b: 2 }


ARITHMETIC OPERATORS

Arithmetic operators apply to numeric values and yield a result of the same type
