---
doc_id: ops/general/docs-introduction
chunk_id: ops/general/docs-introduction#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1035
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
    "b": {
        "c": "foo"
    }
}

could be represented as


Copy code
Copied!

"a": 3
"b": "c": "foo"

All the information of the original JSON file is retained in this
representation.

CUE generalizes this notion to the following pattern:


Copy code
Copied!

<set of nodes>: <constraints>

Each field declaration in CUE defines a set of nodes to which to apply
a specific constraint.
Because order doesn’t matter, multiple constraints can be applied to the
same nodes, all of which need to apply simultaneously.
Such constraints may even be in different files.
But they may never contradict each other:
if one declaration says a field is 5, another may not override it to be 6.
Declaring a field to be both >5 and <10 is valid, though.

This approach is more restricted than full-blown inheritance;
it may not be possible to reuse existing configurations.
On the other hand, it is also a more powerful boilerplate remover.
For instance, suppose each job in a set needs to use a specific
template.
Instead of having to spell this out at each point,
one can declare this separately in a one blanket statement.

So instead of


Copy code
Copied!

jobs: {
	foo: acmeMonitoring & {...}
	bar: acmeMonitoring & {...}
	baz: acmeMonitoring & {...}
}

one can write


Copy code
Copied!

jobs: [string]: acmeMonitoring

jobs: {
	foo: {...}
	bar: {...}
	baz: {...}
}

There is no need to repeat the reference to the monitoring template for
each job, as the first already states that all jobs must use acmeMonitoring.
Such requirements can be specified across files.

This approach not only reduces the boilerplate contained in acmeMonitoring
but also removes the repetitiveness of having to specify
this template for each job in jobs.
At the same time, this statement acts as a type enforcement.
This dual function is a key aspect of CUE and
typed feature structure languages in general.

This approach breaks down, of course, if the restrictions in
acmeMonitoring are too stringent and jobs need to override them.
To this extent, CUE provides mechanisms to allow defaults, opt-out, and
