---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#29-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 518
summary:  * Bytes values are equal if they contain the same sequence of bytes.  * Struct values are equal if they have the same set of regular field labels
---

 * Bytes values are equal if they contain the same sequence of bytes.
 * Struct values are equal if they have the same set of regular field labels
   and the corresponding values are recursively equal. Only regular fields are
   considered; field order and closedness are irrelevant.
 * List values are equal if they have the same length and their corresponding
   elements are recursively equal.

For ordering comparisons (<, <=, >, >=):

 * Numeric values are ordered by their numeric value, with integer-to-float
   conversion as described above.
 * String values are ordered lexically byte-wise.
 * Bytes values are ordered lexically byte-wise.

For pattern matching (=~, !~):

 * The regular expression syntax is that accepted by RE2 (https://github.com/google/re2/wiki/Syntax) [https://github.com/google/re2/wiki/Syntax%29], except for \C.
 * s =~ r is true if string s matches regular expression r.
 * s !~ r is true if string s does not match regular expression r.


Copy code
Copied!

3 < 4       // true
3 < 4.0     // true
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
