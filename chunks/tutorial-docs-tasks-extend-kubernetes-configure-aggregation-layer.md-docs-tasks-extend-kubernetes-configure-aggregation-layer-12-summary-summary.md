---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#12-summary
chunk_level: summary
chunk_type: prose
heading: Authentication Flow
token_count: 128
summary: 1. Kubernetes apiserver: authenticate the requesting user and authorize their rights to the requested API path. 2. Kubernetes apiserver: proxy the request to the extension apiserver 3. Extension...
---

1. Kubernetes apiserver: authenticate the requesting user and authorize their
rights to the requested API path.
2. Kubernetes apiserver: proxy the request to the extension apiserver
3. Extension apiserver: authenticate the request from the Kubernetes apiserver
4. Extension apiserver: authorize the request from the original user
5. Extension apiserver: execute
The rest of this section describes these steps in detail.
The flow can be seen in the following diagram.
![aggregation auth flows](/images/docs/aggregation-api-auth-flow.png)
The source for the above swimlanes can be found in the source of this document.