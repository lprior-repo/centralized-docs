---
doc_id: ref/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you.md/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you
chunk_id: ref/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you.md/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you#7-standard
chunk_level: standard
chunk_type: prose
heading: Feedback
token_count: 449
summary: #### Workaround You can mitigate this issue by using [cAdvisor](https://github.com/google/cadvisor) as a standalone daemonset. 1. Find the latest [cAdvisor...
---

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