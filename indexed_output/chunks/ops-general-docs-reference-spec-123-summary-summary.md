---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#123-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: struct           number of distinct data fields, excluding field constraints. Expression           Result
---

struct           number of distinct data fields, excluding field constraints


Copy code
Copied!

Expression           Result
len("Hellø")         6
len([1, 2, 3])       3
len([1, 2, ...])     2

CLOSE

The builtin function close converts a partially defined, or open, struct
to a fully defined, or closed, struct.

AND

The builtin function and takes a list and returns the result of applying
the & operator to all elements in the list.
It returns top for the empty list.


Copy code
Copied!

Expression:          Result
