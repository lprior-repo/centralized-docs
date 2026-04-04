---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#15-summary
chunk_level: summary
chunk_type: prose
heading: Safe and Unsafe Sysctls
token_count: 113
summary: * `net.ipv4.tcp\_keepalive\_time` (since Kubernetes 1.29, needs kernel 4.5+); * `net.ipv4.tcp\_fin\_timeout` (since Kubernetes 1.29, needs kernel 4.6+); * `net.ipv4.tcp\_keepalive\_intvl` (since...
---

* `net.ipv4.tcp\_keepalive\_time` (since Kubernetes 1.29, needs kernel 4.5+);
* `net.ipv4.tcp\_fin\_timeout` (since Kubernetes 1.29, needs kernel 4.6+);
* `net.ipv4.tcp\_keepalive\_intvl` (since Kubernetes 1.29, needs kernel 4.5+);
* `net.ipv4.tcp\_keepalive\_probes` (since Kubernetes 1.29, needs kernel 4.5+).