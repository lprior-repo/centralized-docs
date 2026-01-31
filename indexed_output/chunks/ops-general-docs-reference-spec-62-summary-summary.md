---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#62-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 133
summary: for any given x. Implementations may error upon encountering a required field constraint
---


for any given x.

Implementations may error upon encountering a required field constraint
when manifesting CUE as data.


Copy code
Copied!

Expression                             Result
{foo?: 3} & {foo: 3}                   {foo: 3}
{foo!: 3} & {foo: 3}                   {foo: 3}

{foo!: int} & {foo: int}               {foo:  int}
{foo!: int} & {foo?: <1}               {foo!: <1}
{foo!: int} & {foo: <=3}               {foo:  <=3}
{foo!: int} & {foo: 3}                 {foo:  3}

{foo!: 3} & {foo: int}                 {foo: 3}
