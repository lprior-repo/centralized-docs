---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#109-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 138
summary: ==    equal. !=    not equal
---



Copy code
Copied!

==    equal
!=    not equal
<     less
<=    less or equal
>     greater
>=    greater or equal
=~    matches regular expression
!~    does not match regular expression

In any comparison, both operands must be concrete; otherwise the result is
bottom (_|_).

The equality operators == and != can be applied to any two concrete
operands.
The ordering operators <, <=, >, and >= apply only to operands of the
same ordered type (numeric, string, or bytes).
The matching operators =~ and !~ apply to a string and a regular expression
