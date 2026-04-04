---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#18-summary
chunk_level: summary
chunk_type: prose
heading: Safe and Unsafe Sysctls
token_count: 125
summary: ### Enabling Unsafe Sysctls All *safe* sysctls are enabled by default. All *unsafe* sysctls are disabled by default and must be allowed manually by the cluster admin on a per-node basis. Pods with...
---

### Enabling Unsafe Sysctls
All *safe* sysctls are enabled by default.
All *unsafe* sysctls are disabled by default and must be allowed manually by the
cluster admin on a per-node basis. Pods with disabled unsafe sysctls will be
scheduled, but will fail to launch.
With the warning above in mind, the cluster admin can allow certain *unsafe*
sysctls for very special situations such as high-performance or real-time
application tuning. *Unsafe* sysctls are enabled on a node-by-node basis with a
flag of the kubelet; for example: