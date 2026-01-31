---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#96-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 133
summary: their operands are not concrete, but define a value that would be legal for. that expression
---

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
