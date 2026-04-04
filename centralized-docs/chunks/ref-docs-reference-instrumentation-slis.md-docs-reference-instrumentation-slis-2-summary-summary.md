---
doc_id: ref/docs-reference-instrumentation-slis.md/docs-reference-instrumentation-slis
chunk_id: ref/docs-reference-instrumentation-slis.md/docs-reference-instrumentation-slis#2-summary
chunk_level: summary
chunk_type: prose
heading: SLI Metrics
token_count: 115
summary: ## SLI Metrics With SLI metrics enabled, each Kubernetes component exposes two metrics, labeled per healthcheck: * a gauge (which represents the current state of the healthcheck) * a counter (which...
---

## SLI Metrics
With SLI metrics enabled, each Kubernetes component exposes two metrics,
labeled per healthcheck:
* a gauge (which represents the current state of the healthcheck)
* a counter (which records the cumulative counts observed for each healthcheck state)
You can use the metric information to calculate per-component availability statistics.
For example, the API server checks the health of etcd. You can work out and report how
available or unavailable etcd has been - as reported by its client, the API server.
The prometheus gauge data looks like this: