---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#4-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1035
summary: Note that it is not an all-or-nothing game. The parallel values are determined on a field-by-field basis
---

Note that it is not an all-or-nothing game.
The parallel values are determined on a field-by-field basis.
So defaults can be selected, or not, independently for fields
that do not depend on each other.

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

This particular case comes in handy in Kubernetes, for instance,
if one wants to equate a set
of labels with a set of selectors
(regardless of whether that is good practice).

But it goes further. Consider


Copy code
Copied!

a: b + 1
b: a - 1
b: 1

When evaluating a, CUE will attempt to resolve b and will find
(a-1) & 1 after unifying the two declarations for b.
It cannot recursively resolve a, as this would result in an
evaluation cycle.
However, the expression (a-1) & 1 is an error
unless (a-1) is 1.
So if this configuration is ever to be a valid, we can safely assume
the answer is 1 and verify that a-1 == 1 after resolving a.

So CUE happily resolves this to


Copy code
Copied!

a: 2
b: 1

without resorting to any fancy algebraic constraint satisfaction solvers,
just plain ol’ logic.
Most cycles that do not result in infinite structures can be handled by CUE.
In fact, it could handle most infinite structures in bounded time
as well, but it puts limits on such cycles for
practical reasons.3

FILE ORGANIZATION

What applies at the language level also applies at the file level.
Within a package, or project, there is no need for files to mutually
import each other.

Files can be split across organizational lines each with a different set
of policies, all implemented with the same familiar constraints.

THE SKY IS THE LIMIT

Many other things are possible.
Take for instance querying.
Whereas validating data is the problem of finding data that is inconsistent with
some constraints,
querying is the problem of finding data that matches some given constraints.
Clearly these two concepts are related.

Computing backwards compatibility (instance of),
computing the most general schema mutually compatible with a set of others
(greatest lower bound),
inferring optimal templates from concrete instances (least upper bound):
