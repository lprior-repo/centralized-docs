---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#51-summary
chunk_level: summary
chunk_type: prose
heading: Protecting cluster components from compromise
token_count: 116
summary: Many third party integrations to Kubernetes may alter the security profile of your cluster. When enabling an integration, always review the permissions that an extension requests before granting it...
---

Many third party integrations to Kubernetes may alter the security profile of your cluster. When
enabling an integration, always review the permissions that an extension requests before granting
it access. For example, many security integrations may request access to view all secrets on
your cluster which is effectively making that component a cluster admin. When in doubt,
restrict the integration to functioning in a single namespace if possible.
Components that create pods may also be unexpectedly powerful if they can do so inside namespaces
like the `kube-system` namespace, because those pods can gain access to service account secrets