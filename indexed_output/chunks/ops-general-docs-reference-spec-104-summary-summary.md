---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#104-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 136
summary: Expression = UnaryExpr | Expression binary_op Expression . UnaryExpr  = PrimaryExpr | unary_op UnaryExpr 
---

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
