---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#15-summary
chunk_level: summary
chunk_type: prose
heading: Controlling access to the Kubernetes API
token_count: 89
summary: ### Use Transport Layer Security (TLS) for all API traffic Kubernetes expects that all API communication in the cluster is encrypted by default with TLS, and the majority of installation methods will...
---

### Use Transport Layer Security (TLS) for all API traffic
Kubernetes expects that all API communication in the cluster is encrypted by default with TLS, and the
majority of installation methods will allow the necessary certificates to be created and distributed to
the cluster components. Note that some components and installation methods may enable local ports over
HTTP and administrators should familiarize themselves with the settings of each component to identify
potentially unsecured traffic.