---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#35-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 145
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
