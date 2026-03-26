---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#37-summary
chunk_level: summary
chunk_type: prose
heading: Enable Kubernetes Apiserver flags
token_count: 127
summary: * `--requestheader-client-ca-file`: When a request arrives to the Kubernetes apiserver, if this option is enabled, the Kubernetes apiserver checks the certificate of the request. If it is signed by...
---

* `--requestheader-client-ca-file`: When a request arrives to the Kubernetes apiserver,
if this option is enabled, the Kubernetes apiserver checks the certificate of the request.
If it is signed by one of the CA certificates in the file reference by `--requestheader-client-ca-file`,
then the request is treated as a potentially legitimate request. The Kubernetes apiserver then
checks if the common name `CN=` is one of the names in the list provided by `--requestheader-allowed-names`.
If the name is allowed, the request is approved; if it is not, the request is not.
If *both*