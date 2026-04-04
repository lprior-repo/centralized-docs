---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#12-summary
chunk_level: summary
chunk_type: prose
heading: Safe and Unsafe Sysctls
token_count: 66
summary: ## Safe and Unsafe Sysctls Kubernetes classes sysctls as either *safe* or *unsafe*. In addition to proper namespacing, a *safe* sysctl must be properly *isolated* between pods on the same node. This...
---

## Safe and Unsafe Sysctls
Kubernetes classes sysctls as either *safe* or *unsafe*. In addition to proper
namespacing, a *safe* sysctl must be properly *isolated* between pods on the
same node. This means that setting a *safe* sysctl for one pod