---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#1-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 513
summary: (a lattice, to be precise). Even entire configurations and schemas are placed in this hierarchy
---

(a lattice, to be precise).
Even entire configurations and schemas are placed in this hierarchy.

WHAT IS A LATTICE?

This section is useful to understand what a lattice is,
but is not strictly needed to grasp the following sections,
nor the specifics of CUE itself. Skip at will.

A lattice is a partially ordered set, in which every two elements
have a unique least upper bound (join) and greatest lower bound (meet).
By definition this means there is always a single root (top) and a single
leaf (bottom).
Let’s consider what this means by looking at an example.
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
are upheld.

An important aspect of a lattice is that for every two elements,
there is a unique instance of both elements that subsumes all other
elements that are an instance of both elements.
This is called the greatest lower bound, or meet.
Now let’s imagine we could define a lattice for, say,
all configurations, schemas and data.
In that case, we could always unambiguously merge two such configurations
independently of order.
This is exactly what CUE does!

CUE’S HIERARCHY

In this section we will introduce CUE’s value hierarchy.
The goal here is to get the big picture, and will only present the details
when it helps for this purpose.


BOOLEANS

Let’s start simple, with booleans.

bool
true
false
⊥ (bottom)

This diagram shows that CUE interprets both true and false as
an instance of bool.
No surprises there.
What is less ordinary is that, to CUE, bool is just as much a value
