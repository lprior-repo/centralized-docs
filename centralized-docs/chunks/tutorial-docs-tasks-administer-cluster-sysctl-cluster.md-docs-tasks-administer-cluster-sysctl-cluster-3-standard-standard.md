---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#3-standard
chunk_level: standard
chunk_type: prose
heading: Safe and Unsafe Sysctls
token_count: 497
summary: ## Safe and Unsafe Sysctls Kubernetes classes sysctls as either *safe* or *unsafe*. In addition to proper namespacing, a *safe* sysctl must be properly *isolated* between pods on the same node. This...
---

## Safe and Unsafe Sysctls
Kubernetes classes sysctls as either *safe* or *unsafe*. In addition to proper
namespacing, a *safe* sysctl must be properly *isolated* between pods on the
same node. This means that setting a *safe* sysctl for one pod
* must not have any influence on any other pod on the node
* must not allow to harm the node's health
* must not allow to gain CPU or memory resources outside of the resource limits
of a pod.
By far, most of the *namespaced* sysctls are not necessarily considered *safe*.
The following sysctls are supported in the *safe* set:
* `kernel.shm\_rmid\_forced`;
* `net.ipv4.ip\_local\_port\_range`;
* `net.ipv4.tcp\_syncookies`;
* `net.ipv4.ping\_group\_range` (since Kubernetes 1.18);
* `net.ipv4.ip\_unprivileged\_port\_start` (since Kubernetes 1.22);
* `net.ipv4.ip\_local\_reserved\_ports` (since Kubernetes 1.27, needs kernel 3.16+);
* `net.ipv4.tcp\_keepalive\_time` (since Kubernetes 1.29, needs kernel 4.5+);
* `net.ipv4.tcp\_fin\_timeout` (since Kubernetes 1.29, needs kernel 4.6+);
* `net.ipv4.tcp\_keepalive\_intvl` (since Kubernetes 1.29, needs kernel 4.5+);
* `net.ipv4.tcp\_keepalive\_probes` (since Kubernetes 1.29, needs kernel 4.5+).
* `net.ipv4.tcp\_rmem` (since Kubernetes 1.32, needs kernel 4.15+).
* `net.ipv4.tcp\_wmem` (since Kubernetes 1.32, needs kernel 4.15+).
#### Note:
There are some exceptions to the set of safe sysctls:
* The `net.\*` sysctls are not allowed with host networking enabled.
* The `net.ipv4.tcp\_syncookies` sysctl is not namespaced on Linux kernel version 4.5 or lower.
This list will be extended in future Kubernetes versions when the kubelet
supports better isolation mechanisms.