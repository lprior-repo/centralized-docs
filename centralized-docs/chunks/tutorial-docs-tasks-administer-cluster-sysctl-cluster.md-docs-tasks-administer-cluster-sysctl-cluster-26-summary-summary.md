---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#26-summary
chunk_level: summary
chunk_type: prose
heading: Setting Sysctls for a Pod
token_count: 125
summary: Due to their nature of being *unsafe*, the use of *unsafe* sysctls is at-your-own-risk and can lead to severe problems like wrong behavior of containers, resource shortage or complete breakage of a...
---

Due to their nature of being *unsafe*, the use of *unsafe* sysctls
is at-your-own-risk and can lead to severe problems like wrong behavior of
containers, resource shortage or complete breakage of a node.
It is good practice to consider nodes with special sysctl settings as
*tainted* within a cluster, and only schedule pods onto them which need those
sysctl settings. It is suggested to use the Kubernetes [*taints and toleration*
feature](/docs/reference/generated/kubectl/kubectl-commands/#taint) to implement this.
A pod with the *unsafe*