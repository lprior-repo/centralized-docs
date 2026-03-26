---
doc_id: tutorial/docs-concepts-cluster-administration-observability.md/docs-concepts-cluster-administration-observability
chunk_id: tutorial/docs-concepts-cluster-administration-observability.md/docs-concepts-cluster-administration-observability#3-standard
chunk_level: standard
chunk_type: prose
heading: Metrics
token_count: 452
summary: ### Metrics tools * [Cortex](https://cortexmetrics.io/) offers horizontally scalable, long-term Prometheus storage. * [Grafana Mimir](https://grafana.com/oss/mimir/) is a Grafana Labs project that...
---

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