---
doc_id: ref/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you.md/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you
chunk_id: ref/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you.md/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you#6-standard
chunk_level: standard
chunk_type: prose
heading: Finding if your app has a dependencies on Docker
token_count: 293
summary: ### Some filesystem metrics are missing and the metrics format is different The Kubelet `/metrics/cadvisor` endpoint provides Prometheus metrics, as documented in [Metrics for Kubernetes system...
---

### Some filesystem metrics are missing and the metrics format is different
The Kubelet `/metrics/cadvisor` endpoint provides Prometheus metrics,
as documented in [Metrics for Kubernetes system components](/docs/concepts/cluster-administration/system-metrics/).
If you install a metrics collector that depends on that endpoint, you might see the following issues:
* The metrics format on the Docker node is `k8s\_&lt;container-name&gt;\_&lt;pod-name&gt;\_&lt;namespace&gt;\_&lt;pod-uid&gt;\_&lt;restart-count&gt;`
but the format on other runtime is different. For example, on containerd node it is `&lt;container-id&gt;`.
* Some filesystem metrics are missing, as follows:
```
`container\_fs\_inodes\_free
container\_fs\_inodes\_total
container\_fs\_io\_current
container\_fs\_io\_time\_seconds\_total
container\_fs\_io\_time\_weighted\_seconds\_total
container\_fs\_limit\_bytes
container\_fs\_read\_seconds\_total
container\_fs\_reads\_merged\_total
container\_fs\_sector\_reads\_total
container\_fs\_sector\_writes\_total
container\_fs\_usage\_bytes
container\_fs\_write\_seconds\_total
container\_fs\_writes\_merged\_total
`
```