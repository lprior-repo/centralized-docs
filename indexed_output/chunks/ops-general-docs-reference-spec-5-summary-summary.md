---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#5-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 138
summary: Group       = \"(\" Expression \")\" . Option      = \"[\" Expression \"]\" 
---

Group       = "(" Expression ")" .
Option      = "[" Expression "]" .
Repetition  = "{" Expression "}" .

Productions are expressions constructed from terms and the following operators,
in increasing precedence:


Copy code
Copied!

|   alternation
()  grouping
[]  option (0 or 1 times)
{}  repetition (0 to n times)

Lower-case production names are used to identify lexical tokens. Non-terminals
are in CamelCase. Lexical tokens are enclosed in double quotes "" or back
quotes ``.

The form a … b represents the set of characters from a through b as
