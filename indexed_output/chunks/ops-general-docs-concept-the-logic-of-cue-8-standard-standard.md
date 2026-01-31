---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#8-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 522
summary: REASONING AND INFERENCE. The values lattice brings CUE another advantage: the ability to reason about
---


REASONING AND INFERENCE

The values lattice brings CUE another advantage: the ability to reason about
values, schemas, and constraints.

We already discussed how limiting inheritance,
whether language-based or file-based,
makes it easier for people to reason about values.
But it also makes it easier for machines.

BOILERPLATE REMOVAL

CUE’s severe restrictions on inheritance limit its
ability to define hierarchies of templates to remove boilerplate.
But CUE provides some new mechanisms for removing boilerplate.

Suppose a node must inherit from multiple templates, or mixins.
Because order is irrelevant in CUE,
there is no need to specify these in a particular order or even in one location.
One can even say on a single line that a collection of
fields must mix in a template.
For instance,


Copy code
Copied!

jobs: [string]: acmeMonitoring

tells CUE that all jobs in jobs must mix in acmeMonitoring.
There is no need to repeat this at every node.

In CUE, though, we typically refer to acmeMonitoring as a constraint.
After all, applying it will guarantee
that a job implements monitoring in a certain way.
If such a constraint also contains sensible defaults, however,
it simultaneously validates and reduces boilerplate.2

This ability to simultaneously
enforce constraints and remove boilerplate
was a key factor in the success of
the typed feature structure systems that inspired the creation of CUE.

This property is also useful in automation.
The cue trim tool can automatically remove boilerplate from configurations
using the same logic.

CYCLES

An astute reader may have noticed that there were cyclic references
between fields in some of the examples,
something that is not allowed in your typical programming or
configuration language.
CUE’s underlying model allows reasoning over cycles.
Consider a CUE program defining two fields;


Copy code
Copied!

a: b
b: a

This can only be interpreted to mean that a and b must be equal.
If there is no concrete value assigned to a or b,
they remain unspecified in the same way as if each had been declared as string.
