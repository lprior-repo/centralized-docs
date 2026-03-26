---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#38-summary
chunk_level: summary
chunk_type: prose
heading: Enable Kubernetes Apiserver flags
token_count: 125
summary: . If the name is allowed, the request is approved; if it is not, the request is not. If *both* `--client-ca-file` and `--requestheader-client-ca-file` are provided, then the request first checks the...
---

.
If the name is allowed, the request is approved; if it is not, the request is not.
If *both* `--client-ca-file` and `--requestheader-client-ca-file` are provided,
then the request first checks the `--requestheader-client-ca-file` CA and then the
`--client-ca-file`. Normally, different CAs, either root CAs or intermediate CAs,
are used for each of these options; regular client requests match against `--client-ca-file`,
while aggregation requests match against `--requestheader-client-ca-file`. However,
if both use the *same*