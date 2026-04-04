---
doc_id: ref/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you.md/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you
chunk_id: ref/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you.md/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you#11-summary
chunk_level: summary
chunk_type: prose
heading: Finding if your app has a dependencies on Docker
token_count: 69
summary: ### Some filesystem metrics are missing and the metrics format is different The Kubelet `/metrics/cadvisor` endpoint provides Prometheus metrics, as documented in [Metrics for Kubernetes system...
---

### Some filesystem metrics are missing and the metrics format is different
The Kubelet `/metrics/cadvisor` endpoint provides Prometheus metrics,
as documented in [Metrics for Kubernetes system components](/docs/concepts/cluster-administration/system-metrics/).
If you install a metrics collector that depends on that endpoint, you might see the following issues: