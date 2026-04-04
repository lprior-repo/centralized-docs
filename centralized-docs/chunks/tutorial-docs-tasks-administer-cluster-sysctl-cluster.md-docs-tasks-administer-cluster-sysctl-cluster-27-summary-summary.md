---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#27-summary
chunk_level: summary
chunk_type: prose
heading: Setting Sysctls for a Pod
token_count: 108
summary: to implement this. A pod with the *unsafe* sysctls will fail to launch on any node which has not enabled those two *unsafe* sysctls explicitly. As with *node-level* sysctls it is recommended to use...
---

 to implement this.
A pod with the *unsafe* sysctls will fail to launch on any node which has not
enabled those two *unsafe* sysctls explicitly. As with *node-level* sysctls it
is recommended to use
[*taints and toleration* feature](/docs/reference/generated/kubectl/kubectl-commands/#taint) or
[taints on nodes](/docs/concepts/scheduling-eviction/taint-and-toleration/)
to schedule those pods onto the right nodes.