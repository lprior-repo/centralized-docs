---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#113-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 134
summary: null == 2   // false. null != {}  // true
---

null == 2   // false
null != {}  // true
{} == {}    // _|_: structs are not comparable against structs

"Wild cats" =~ "cat"   // true
"Wild cats" !~ "dog"   // true

"foo" =~ "^[a-z]{3}$"  // true
"foo" =~ "^[a-z]{4}$"  // false


LOGICAL OPERATORS

Logical operators apply to boolean values and yield a result of the same type
as the operands. The right operand is evaluated conditionally.


Copy code
Copied!

&&    conditional AND    p && q  is  "if p then q else false"
||    conditional OR     p || q  is  "if p then true else q"
