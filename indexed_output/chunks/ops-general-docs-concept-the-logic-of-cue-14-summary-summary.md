---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#14-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary: Below are two examples of an ordering defined on structs. municipality
---


Below are two examples of an ordering defined on structs.

municipality
name: string
population: int
big city
name: string
population: >1M
London
name: 'London'
population: 8M
London is a big city, which is a municipality
⊤
a: int
b: int
a: 1
a: int
b: int
a: 1
b: 1
⊥
b: 1

Loosely speaking, a struct is an instance of another if it has at least
all the fields defined by the parent and if its constraints on these fields
are at least as strict as those defined by its parent.

The instance relation for structs has an analogy in
