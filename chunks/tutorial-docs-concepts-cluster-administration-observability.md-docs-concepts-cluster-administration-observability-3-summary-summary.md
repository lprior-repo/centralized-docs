---
doc_id: tutorial/docs-concepts-cluster-administration-observability.md/docs-concepts-cluster-administration-observability
chunk_id: tutorial/docs-concepts-cluster-administration-observability.md/docs-concepts-cluster-administration-observability#3-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 104
summary: Figure 1 outlines how cluster components emit the three primary signal types. flowchart LR A[Cluster components] --&gt; M[Metrics pipeline] A --&gt; L[Log pipeline] A --&gt; T[Trace pipeline] M...
---

Figure 1 outlines how cluster components emit the three primary signal types.
flowchart LR
A[Cluster components] --&gt; M[Metrics pipeline]
A --&gt; L[Log pipeline]
A --&gt; T[Trace pipeline]
M --&gt; S[(Storage and analysis)]
L --&gt; S
T --&gt; S
S --&gt; O[Operators and automation]
*Figure 1. High-level signals emitted by cluster components and their consumers.*