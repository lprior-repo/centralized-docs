---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#63-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 133
summary: {foo!: 3} & {foo: <=4}                 {foo: 3}. {foo?: 1} & {foo?: 2}                  {foo?: _|_} // No error
---

{foo!: 3} & {foo: <=4}                 {foo: 3}

{foo?: 1} & {foo?: 2}                  {foo?: _|_} // No error
{foo?: 1} & {foo!: 2}                  _|_
{foo?: 1} & {foo: 2}                   _|_


DYNAMIC FIELDS

A dynamic field is a field whose label is determined by
an expression wrapped in parentheses.
A dynamic field may be marked as optional or required.


Copy code
Copied!

Expression                             Result
a:   "foo"                             a:   "foo"
b:   "bar"                             b:   "bar"
