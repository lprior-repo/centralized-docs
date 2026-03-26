---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#39-summary
chunk_level: summary
chunk_type: prose
heading: Enable Kubernetes Apiserver flags
token_count: 120
summary: `--client-ca-file`, while aggregation requests match against `--requestheader-client-ca-file`. However, if both use the *same* CA, then client requests that normally would pass via `--client-ca-file`...
---

`--client-ca-file`,
while aggregation requests match against `--requestheader-client-ca-file`. However,
if both use the *same* CA, then client requests that normally would pass via `--client-ca-file`
will fail, because the CA will match the CA in `--requestheader-client-ca-file`,
but the common name `CN=` will **not** match one of the acceptable common names in
`--requestheader-allowed-names`. This can cause your kubelets and other control plane components,
as well as end-users, to be unable to authenticate to the Kubernetes apiserver.