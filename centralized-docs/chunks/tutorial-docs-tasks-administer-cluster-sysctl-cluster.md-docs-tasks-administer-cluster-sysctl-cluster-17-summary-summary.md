---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#17-summary
chunk_level: summary
chunk_type: prose
heading: Safe and Unsafe Sysctls
token_count: 81
summary: #### Note: There are some exceptions to the set of safe sysctls: * The `net.\*` sysctls are not allowed with host networking enabled. * The `net.ipv4.tcp\_syncookies` sysctl is not namespaced on...
---

#### Note:
There are some exceptions to the set of safe sysctls:
* The `net.\*` sysctls are not allowed with host networking enabled.
* The `net.ipv4.tcp\_syncookies` sysctl is not namespaced on Linux kernel version 4.5 or lower.
This list will be extended in future Kubernetes versions when the kubelet
supports better isolation mechanisms.