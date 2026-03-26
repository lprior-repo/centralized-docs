---
doc_id: ref/docs-concepts-cluster-administration-kube-state-metrics.md/docs-concepts-cluster-administration-kube-state-metrics
chunk_id: ref/docs-concepts-cluster-administration-kube-state-metrics.md/docs-concepts-cluster-administration-kube-state-metrics#6-summary
chunk_level: summary
chunk_type: prose
heading: Example: alerting based on from kube-state-metrics
token_count: 73
summary: ## Example: alerting based on from kube-state-metrics Metrics generated from kube-state-metrics also allow for alerting on issues in the cluster. If you use Prometheus or a similar tool that uses the...
---

## Example: alerting based on from kube-state-metrics
Metrics generated from kube-state-metrics also allow for alerting on issues in the cluster.
If you use Prometheus or a similar tool that uses the same alert rule language, the following alert will fire if there are pods that have been in a `Terminating` state for more than 5 minutes: