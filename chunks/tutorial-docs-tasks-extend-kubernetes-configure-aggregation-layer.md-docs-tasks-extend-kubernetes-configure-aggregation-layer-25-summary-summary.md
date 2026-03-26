---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#25-summary
chunk_level: summary
chunk_type: prose
heading: Authentication Flow
token_count: 77
summary: ### Extension Apiserver Authenticates the Request The extension apiserver, upon receiving a proxied request from the Kubernetes apiserver, must validate that the request actually did come from a...
---

### Extension Apiserver Authenticates the Request
The extension apiserver, upon receiving a proxied request from the Kubernetes apiserver,
must validate that the request actually did come from a valid authenticating proxy,
which role the Kubernetes apiserver is fulfilling. The extension apiserver validates it via:
1. Retrieve the following from the configmap in `kube-system`, as described above: