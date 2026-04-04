---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#20-summary
chunk_level: summary
chunk_type: prose
heading: Setting Sysctls for a Pod
token_count: 84
summary: ## Setting Sysctls for a Pod A number of sysctls are *namespaced* in today's Linux kernels. This means that they can be set independently for each pod on a node. Only namespaced sysctls are...
---

## Setting Sysctls for a Pod
A number of sysctls are *namespaced* in today's Linux kernels. This means that
they can be set independently for each pod on a node. Only namespaced sysctls
are configurable via the pod securityContext within Kubernetes.
The following sysctls are known to be namespaced. This list could change
in future versions of the Linux kernel.