---
doc_id: ref/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you.md/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you
chunk_id: ref/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you.md/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 845
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
#### Workaround
You can mitigate this issue by using [cAdvisor](https://github.com/google/cadvisor) as a standalone daemonset.
1. Find the latest [cAdvisor release](https://github.com/google/cadvisor/releases)
with the name pattern `vX.Y.Z-containerd-cri` (for example, `v0.42.0-containerd-cri`).
2. Follow the steps in [cAdvisor Kubernetes Daemonset](https://github.com/google/cadvisor/tree/master/deploy/kubernetes) to create the daemonset.
3. Point the installed metrics collector to use the cAdvisor `/metrics` endpoint
which provides the full set of
[Prometheus container metrics](https://github.com/google/cadvisor/blob/master/docs/storage/prometheus.md).
Alternatives:
* Use alternative third party metrics collection solution.
* Collect metrics from the Kubelet summary API that is served at `/stats/summary`.## What's next
* Read [Migrating from dockershim](/docs/tasks/administer-cluster/migrating-from-dockershim/) to understand your next steps
* Read the [dockershim deprecation FAQ](/blog/2020/12/02/dockershim-faq/) article for more information.
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified March 23, 2023 at 11:16 AM PST: [Apply suggestions from code review (9b78ecff2c)](https://github.com/kubernetes/website/commit/9b78ecff2cceb92fe0fef29c4a1188fb70eda22d)
## Related Pages

- [Communication between Nodes and the Control Plane](docs-concepts-architecture-control-plane-node-communication.md)
- [Kubernetes Scheduler](docs-concepts-scheduling-eviction-kube-scheduler.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)
- [HorizontalPodAutoscaler](docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)