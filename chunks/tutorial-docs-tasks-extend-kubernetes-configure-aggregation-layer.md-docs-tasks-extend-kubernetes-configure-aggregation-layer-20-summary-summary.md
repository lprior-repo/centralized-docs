---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#20-summary
chunk_level: summary
chunk_type: prose
heading: Authentication Flow
token_count: 58
summary: 1. The connection must be made using a client certificate that is signed by the CA whose certificate is in `--requestheader-client-ca-file`. 2. The connection must be made using a client certificate...
---

1. The connection must be made using a client certificate that is signed by
the CA whose certificate is in `--requestheader-client-ca-file`.
2. The connection must be made using a client certificate whose CN is one of
those listed in `--requestheader-allowed-names`.