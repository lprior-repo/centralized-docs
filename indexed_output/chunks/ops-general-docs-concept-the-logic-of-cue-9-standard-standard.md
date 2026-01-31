---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#9-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 520
summary: This particular case comes in handy in Kubernetes, for instance,. if one wants to equate a set
---


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
all of these fall in the realm of possibilities of CUE’s model.

REFERENCES

The title of this section refers to Bob Carpenter’s
“The Logic of Typed Feature Structures”
(1992, Cambridge University Press, ISBN:0-521-41932-8).
