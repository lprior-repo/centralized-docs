---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#22-summary
chunk_level: summary
chunk_type: prose
heading: Authentication Flow
token_count: 125
summary: 1. Use them to authenticate to the extension apiserver. 2. Create a configmap in the `kube-system` namespace called `extension-apiserver-authentication`, in which it will place the CA certificate and...
---

1. Use them to authenticate to the extension apiserver.
2. Create a configmap in the `kube-system` namespace called `extension-apiserver-authentication`,
in which it will place the CA certificate and the allowed CNs. These in turn can be retrieved
by extension apiservers to validate requests.
Note that the same client certificate is used by the Kubernetes apiserver to authenticate
against *all* extension apiservers. It does not create a client certificate per extension
apiserver, but rather a single one to authenticate as the Kubernetes apiserver.
This same one is reused for all extension apiserver requests.