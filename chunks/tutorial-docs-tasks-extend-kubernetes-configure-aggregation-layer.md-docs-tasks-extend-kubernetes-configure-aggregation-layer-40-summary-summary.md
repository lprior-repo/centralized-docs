---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#40-summary
chunk_level: summary
chunk_type: prose
heading: Enable Kubernetes Apiserver flags
token_count: 76
summary: . This can cause your kubelets and other control plane components, as well as end-users, to be unable to authenticate to the Kubernetes apiserver. For this reason, use different CA certs for the...
---

. This can cause your kubelets and other control plane components,
as well as end-users, to be unable to authenticate to the Kubernetes apiserver.
For this reason, use different CA certs for the `--client-ca-file`
option - to authorize control plane components and end-users - and the `--requestheader-client-ca-file` option - to authorize aggregation apiserver requests.