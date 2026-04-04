---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#13-summary
chunk_level: summary
chunk_type: prose
heading: Safe and Unsafe Sysctls
token_count: 114
summary: * must not have any influence on any other pod on the node * must not allow to harm the node's health * must not allow to gain CPU or memory resources outside of the resource limits of a pod. By far,...
---

* must not have any influence on any other pod on the node
* must not allow to harm the node's health
* must not allow to gain CPU or memory resources outside of the resource limits
of a pod.
By far, most of the *namespaced* sysctls are not necessarily considered *safe*.
The following sysctls are supported in the *safe* set:
* `kernel.shm\_rmid\_forced`;
* `net.ipv4.ip\_local\_port\_range`;
* `net.ipv4.tcp\_syncookies`;