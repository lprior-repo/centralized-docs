---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#36-summary
chunk_level: summary
chunk_type: prose
heading: Enable Kubernetes Apiserver flags
token_count: 115
summary: * `--client-ca-file`: When a request arrives to the Kubernetes apiserver, if this option is enabled, the Kubernetes apiserver checks the certificate of the request. If it is signed by one of the CA...
---

* `--client-ca-file`: When a request arrives to the Kubernetes apiserver,
if this option is enabled, the Kubernetes apiserver checks the certificate
of the request. If it is signed by one of the CA certificates in the file referenced by
`--client-ca-file`, then the request is treated as a legitimate request,
and the user is the value of the common name `CN=`, while the group is the organization `O=`.
See the [documentation on TLS authentication](/docs/reference/access-authn-authz/authentication/#x509-client-certificates).