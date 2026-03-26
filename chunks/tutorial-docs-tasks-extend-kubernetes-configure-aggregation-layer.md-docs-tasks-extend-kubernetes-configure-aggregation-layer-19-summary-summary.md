---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#19-summary
chunk_level: summary
chunk_type: prose
heading: Authentication Flow
token_count: 118
summary: * private key file via `--proxy-client-key-file` * signed client certificate file via `--proxy-client-cert-file` * certificate of the CA that signed the client certificate file via...
---

* private key file via `--proxy-client-key-file`
* signed client certificate file via `--proxy-client-cert-file`
* certificate of the CA that signed the client certificate file via `--requestheader-client-ca-file`
* valid Common Name values (CNs) in the signed client certificate via `--requestheader-allowed-names`
The Kubernetes apiserver will use the files indicated by `--proxy-client-\*-file`
to authenticate to the extension apiserver. In order for the request to be considered
valid by a compliant extension apiserver, the following conditions must be met: