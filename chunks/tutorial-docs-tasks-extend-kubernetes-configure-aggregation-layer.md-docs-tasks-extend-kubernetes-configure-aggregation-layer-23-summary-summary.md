---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#23-summary
chunk_level: summary
chunk_type: prose
heading: Authentication Flow
token_count: 72
summary: #### Original Request Username and Group When the Kubernetes apiserver proxies the request to the extension apiserver, it informs the extension apiserver of the username and group with which the...
---

#### Original Request Username and Group
When the Kubernetes apiserver proxies the request to the extension apiserver,
it informs the extension apiserver of the username and group with which the
original request successfully authenticated. It provides these in http headers
of its proxied request. You must inform the Kubernetes apiserver of the names
of the headers to be used.