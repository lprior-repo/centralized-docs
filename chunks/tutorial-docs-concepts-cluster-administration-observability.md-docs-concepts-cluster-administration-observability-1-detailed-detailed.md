---
doc_id: tutorial/docs-concepts-cluster-administration-observability.md/docs-concepts-cluster-administration-observability
chunk_id: tutorial/docs-concepts-cluster-administration-observability.md/docs-concepts-cluster-administration-observability#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Metrics
token_count: 970
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
### Metrics tools
* [Cortex](https://cortexmetrics.io/) offers horizontally scalable, long-term Prometheus storage.
* [Grafana Mimir](https://grafana.com/oss/mimir/) is a Grafana Labs project that provides multi-tenant, horizontally scalable Prometheus-compatible storage.
* [Prometheus](https://prometheus.io/) is the monitoring system that scrapes and stores metrics from Kubernetes components.
* [Thanos](https://thanos.io/) extends Prometheus with global querying, downsampling, and object storage support.### Logging tools
* [Elasticsearch](https://www.elastic.co/elasticsearch/) delivers distributed log indexing and search.
* [Fluent Bit](https://fluentbit.io/) collects and forwards container and node logs with a low resource footprint.
* [Fluentd](https://www.fluentd.org/) routes and transforms logs to multiple destinations.
* [Grafana Loki](https://grafana.com/oss/loki/) stores logs in a Prometheus-inspired, label-based format.
* [OpenSearch](https://opensearch.org/) provides open source log indexing and search compatible with Elasticsearch APIs.### Tracing tools
* [Grafana Tempo](https://grafana.com/oss/tempo/) offers scalable, low-cost distributed tracing storage.
* [Jaeger](https://www.jaegertracing.io/) captures and visualizes distributed traces for microservices.
* [OpenTelemetry Collector](https://opentelemetry.io/docs/collector/) receives, processes, and exports telemetry data including traces.
* [Zipkin](https://zipkin.io/) provides distributed tracing collection and visualization.## What's next
* Learn how to [collect resource usage metrics with metrics-server](/docs/tasks/debug/debug-cluster/resource-usage-monitoring/)
* Explore [logging tasks and tutorials](/docs/tasks/debug/logging/)
* Follow the [monitoring and tracing task guides](/docs/tasks/debug/monitoring/)
* Review the [system metrics guide](/docs/concepts/cluster-administration/system-metrics/) for component endpoints and stability
* Review the [common observability tools](#common-observability-tools) section for vetted third-party options