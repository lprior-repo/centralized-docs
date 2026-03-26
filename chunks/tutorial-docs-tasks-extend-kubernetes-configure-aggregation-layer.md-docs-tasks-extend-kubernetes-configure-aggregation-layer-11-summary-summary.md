---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#11-summary
chunk_level: summary
chunk_type: prose
heading: Authentication Flow
token_count: 117
summary: ## Authentication Flow Unlike Custom Resource Definitions (CRDs), the Aggregation API involves another server - your Extension apiserver - in addition to the standard Kubernetes apiserver. The...
---

## Authentication Flow
Unlike Custom Resource Definitions (CRDs), the Aggregation API involves
another server - your Extension apiserver - in addition to the standard Kubernetes apiserver.
The Kubernetes apiserver will need to communicate with your extension apiserver,
and your extension apiserver will need to communicate with the Kubernetes apiserver.
In order for this communication to be secured, the Kubernetes apiserver uses x509
certificates to authenticate itself to the extension apiserver.
This section describes how the authentication and authorization flows work,
and how to configure them.
The high-level flow is as follows: