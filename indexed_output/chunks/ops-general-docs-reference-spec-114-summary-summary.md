---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#114-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 145
summary: !     NOT                !p      is  \"not p\". Calls can be made to core library functions, called builtins
---

!     NOT                !p      is  "not p"

CALLS

Calls can be made to core library functions, called builtins.
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
