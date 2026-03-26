---
doc_id: tutorial/docs-concepts-cluster-administration-observability.md/docs-concepts-cluster-administration-observability
chunk_id: tutorial/docs-concepts-cluster-administration-observability.md/docs-concepts-cluster-administration-observability#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 228
summary: # Observability Understand how to gain end-to-end visibility of a Kubernetes cluster through the collection of metrics, logs, and traces. In Kubernetes, observability is the process of collecting and...
---

# Observability
Understand how to gain end-to-end visibility of a Kubernetes cluster through the collection of metrics, logs, and traces.
In Kubernetes, observability is the process of collecting and analyzing metrics, logs, and traces—often referred to as the three pillars of observability—in order to obtain a better understanding of the internal state, performance, and health of the cluster.
Kubernetes control plane components, as well as many add-ons, generate and emit these signals. By aggregating and correlating them, you can gain a unified picture of the control plane, add-ons, and applications across the cluster.
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