---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#41-summary
chunk_level: summary
chunk_type: prose
heading: Enable Kubernetes Apiserver flags
token_count: 82
summary: #### Warning: Do **not** reuse a CA that is used in a different context unless you understand the risks and the mechanisms to protect the CA's usage. If you are not running kube-proxy on a host...
---

#### Warning:
Do **not** reuse a CA that is used in a different context unless you understand
the risks and the mechanisms to protect the CA's usage.
If you are not running kube-proxy on a host running the API server,
then you must make sure that the system is enabled with the following
`kube-apiserver` flag:
```
`--enable-aggregator-routing=true
`
```