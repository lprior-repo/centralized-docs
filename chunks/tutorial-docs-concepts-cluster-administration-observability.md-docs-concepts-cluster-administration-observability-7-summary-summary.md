---
doc_id: tutorial/docs-concepts-cluster-administration-observability.md/docs-concepts-cluster-administration-observability
chunk_id: tutorial/docs-concepts-cluster-administration-observability.md/docs-concepts-cluster-administration-observability#7-summary
chunk_level: summary
chunk_type: prose
heading: Metrics
token_count: 120
summary: for details and configuration options. Figure 2 outlines a common Kubernetes metrics pipeline. flowchart LR C[Cluster components] --&gt; P[Prometheus scraper] P --&gt; TS[(Time series storage)] TS...
---

 for details and configuration options.
Figure 2 outlines a common Kubernetes metrics pipeline.
flowchart LR
C[Cluster components] --&gt; P[Prometheus scraper]
P --&gt; TS[(Time series storage)]
TS --&gt; D[Dashboards and alerts]
TS --&gt; A[Automated actions]
*Figure 2. Components of a typical Kubernetes metrics pipeline.*
For multi-cluster or multi-cloud visibility, distributed time series databases (for example Thanos or Cortex) can complement Prometheus.
See [Common observability tools - metrics tools](#metrics-tools)