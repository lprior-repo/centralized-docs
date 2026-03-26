---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#17-summary
chunk_level: summary
chunk_type: prose
heading: Authentication Flow
token_count: 86
summary: 1. How should the Kubernetes apiserver authenticate to the extension apiserver, informing the extension apiserver that the request, which comes over the network, is coming from a valid Kubernetes...
---

1. How should the Kubernetes apiserver authenticate to the extension apiserver,
informing the extension apiserver that the request, which comes over the network,
is coming from a valid Kubernetes apiserver?
2. How should the Kubernetes apiserver inform the extension apiserver of the
username and group for which the original request was authenticated?
In order to provide for these two, you must configure the Kubernetes apiserver using several flags.