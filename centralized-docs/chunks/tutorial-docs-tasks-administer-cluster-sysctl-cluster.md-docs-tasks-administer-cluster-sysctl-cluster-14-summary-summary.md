---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#14-summary
chunk_level: summary
chunk_type: prose
heading: Safe and Unsafe Sysctls
token_count: 121
summary: * `net.ipv4.ip\_local\_port\_range`; * `net.ipv4.tcp\_syncookies`; * `net.ipv4.ping\_group\_range` (since Kubernetes 1.18); * `net.ipv4.ip\_unprivileged\_port\_start` (since Kubernetes 1.22); *...
---

* `net.ipv4.ip\_local\_port\_range`;
* `net.ipv4.tcp\_syncookies`;
* `net.ipv4.ping\_group\_range` (since Kubernetes 1.18);
* `net.ipv4.ip\_unprivileged\_port\_start` (since Kubernetes 1.22);
* `net.ipv4.ip\_local\_reserved\_ports` (since Kubernetes 1.27, needs kernel 3.16+);
* `net.ipv4.tcp\_keepalive\_time` (since Kubernetes 1.29, needs kernel 4.5+);