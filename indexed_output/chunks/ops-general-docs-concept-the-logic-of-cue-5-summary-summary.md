---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#5-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 145
summary: This diagrams below show a lattice of all values of respectively a. 2- and 3- element set, ordered by the subset relation
---

This diagrams below show a lattice of all values of respectively a
2- and 3- element set, ordered by the subset relation.

{x, y}
{x}
{y}
{}
{x, y, z}
{x, y}
{x, z}
{y, z}
{x}
{y}
{z}
{}
Squint harder if you can't recognize the cube.

If an element B is a subset of element A, there is a path from A to B.
In more general terms, we then say that A _subsumes_ B, or that
B is an _instance of_ A.
In our examples, `{x}` is an instance of `{x, y}`,
because we defined our lattice to use the subset relation.
But we can use any relation we want as long as the properties of a lattice
