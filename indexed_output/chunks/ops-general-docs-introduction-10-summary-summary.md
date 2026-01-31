---
doc_id: ops/general/docs-introduction
chunk_id: ops/general/docs-introduction#10-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 142
summary: PUSH, NOT PULL, CONSTRAINTS. CUE’s constraints act as data validators, but also double as
---

remains.

PUSH, NOT PULL, CONSTRAINTS

CUE’s constraints act as data validators, but also double as
a mechanism to reduce boilerplate.
This is a powerful approach, but requires some different thinking.
With traditional inheritance approaches one specifies the templates that
are to be inherited from at each point they should be used.
In CUE, instead, one selects a set of nodes in the configuration to which
to apply a template.
This selection can be at a different point in the configuration altogether.

Another way to view this, a JSON configuration, say, can be
