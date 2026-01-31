---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#32-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: Suppose a node must inherit from multiple templates, or mixins. Because order is irrelevant in CUE,
---


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
