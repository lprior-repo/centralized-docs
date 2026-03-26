---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#10-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 116
summary: #### Note: There are a few setup requirements for getting the aggregation layer working in your environment to support mutual TLS auth between the proxy and extension apiservers. Kubernetes and the...
---

#### Note:
There are a few setup requirements for getting the aggregation layer working in
your environment to support mutual TLS auth between the proxy and extension apiservers.
Kubernetes and the kube-apiserver have multiple CAs, so make sure that the proxy is
signed by the aggregation layer CA and not by something else, like the Kubernetes general CA.
#### Caution:
Reusing the same CA for different client types can negatively impact the cluster's
ability to function. For more information, see [CA Reusage and Conflicts](#ca-reusage-and-conflicts).