---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#99-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 139
summary:     greeting: \"Hello, \(place)!\". b: a & { place: \"world\" }
---

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
