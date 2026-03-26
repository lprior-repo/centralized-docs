---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#24-summary
chunk_level: summary
chunk_type: prose
heading: Authentication Flow
token_count: 87
summary: * the header in which to store the username via `--requestheader-username-headers` * the header in which to store the group via `--requestheader-group-headers` * the prefix to append to all extra...
---

* the header in which to store the username via `--requestheader-username-headers`
* the header in which to store the group via `--requestheader-group-headers`
* the prefix to append to all extra headers via `--requestheader-extra-headers-prefix`
These header names are also placed in the `extension-apiserver-authentication` configmap,
so they can be retrieved and used by extension apiservers.