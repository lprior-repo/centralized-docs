---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#105-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 130
summary: There are eight precedence levels for binary operators. Multiplication operators binds strongest, followed by
---


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
