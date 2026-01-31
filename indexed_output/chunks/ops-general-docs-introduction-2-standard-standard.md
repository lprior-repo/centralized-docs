---
doc_id: ops/general/docs-introduction
chunk_id: ops/general/docs-introduction#2-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 514
summary: also makes an effort to incorporate lessons learned from 15 years of GCL usage. This also includes lessons learned from offsprings and different approaches to
---

also makes an effort to incorporate lessons learned from 15 years of GCL usage.
This also includes lessons learned from offsprings and different approaches to
configuration altogether.

PHILOSOPHY AND PRINCIPLES

TYPES ARE VALUES

CUE does not distinguish between values and types.
This is a powerful notion that allows CUE to define ultra-detailed
constraints, but it also simplifies things considerably:
there is no separate schema or data definition language to learn
and related language constructs such as sum types, enums,
and even null coalescing collapse onto a single construct.

Below is a demonstration of this concept.
On the left one can see a JSON object (in CUE syntax) with some properties
about the city of Moscow.
The middle column shows a possible schema for any municipality.
On the right one sees a mix between data and schema as is exemplary of CUE.

Data


Copy code
Copied!

moscow: {
	name:    "Moscow"
	pop:     11.92M
	capital: true
}

Schema


Copy code
Copied!

municipality: {
	name:    string
	pop:     int
	capital: bool
}

CUE


Copy code
Copied!

largeCapital: {
	name:    string
	pop:     >5M
	capital: true
}

In general, in CUE one starts with a broad definition of a type, describing
all possible instances.
One then narrows down these definitions, possibly by combining constraints
from different sources (departments, users), until a concrete data instance
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
defined as a sequence of path-leaf values.
For instance,


Copy code
Copied!

{
    "a": 3,
