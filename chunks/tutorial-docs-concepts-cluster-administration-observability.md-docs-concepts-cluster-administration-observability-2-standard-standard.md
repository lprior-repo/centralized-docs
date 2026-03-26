---
doc_id: tutorial/docs-concepts-cluster-administration-observability.md/docs-concepts-cluster-administration-observability
chunk_id: tutorial/docs-concepts-cluster-administration-observability.md/docs-concepts-cluster-administration-observability#2-standard
chunk_level: standard
chunk_type: prose
heading: Metrics
token_count: 290
summary: ## Metrics Kubernetes components emit metrics in [Prometheus format](https://prometheus.io/docs/instrumenting/exposition_formats/) from their `/metrics` endpoints, including: *...
---

## Metrics
Kubernetes components emit metrics in [Prometheus format](https://prometheus.io/docs/instrumenting/exposition_formats/) from their `/metrics` endpoints, including:
* kube-controller-manager
* kube-proxy
* kube-apiserver
* kube-scheduler
* kubelet
The kubelet also exposes metrics at `/metrics/cadvisor`, `/metrics/resource`, and `/metrics/probes`, and add-ons such as [kube-state-metrics](/docs/concepts/cluster-administration/kube-state-metrics/) enrich those control plane signals with Kubernetes object status.
A typical Kubernetes metrics pipeline periodically scrapes these endpoints and stores the samples in a time series database (for example with Prometheus).
See the [system metrics guide](/docs/concepts/cluster-administration/system-metrics/) for details and configuration options.
Figure 2 outlines a common Kubernetes metrics pipeline.
flowchart LR
C[Cluster components] --&gt; P[Prometheus scraper]
P --&gt; TS[(Time series storage)]
TS --&gt; D[Dashboards and alerts]
TS --&gt; A[Automated actions]
*Figure 2. Components of a typical Kubernetes metrics pipeline.*
For multi-cluster or multi-cloud visibility, distributed time series databases (for example Thanos or Cortex) can complement Prometheus.
See [Common observability tools - metrics tools](#metrics-tools) for metrics scrapers and time series databases.