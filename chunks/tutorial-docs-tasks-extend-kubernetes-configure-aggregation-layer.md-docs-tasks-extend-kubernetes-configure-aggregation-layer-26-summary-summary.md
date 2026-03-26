---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#26-summary
chunk_level: summary
chunk_type: prose
heading: Authentication Flow
token_count: 80
summary: * Client CA certificate * List of allowed names (CNs) * Header names for username, group and extra info * Check that the TLS connection was authenticated using a client certificate which: * Was...
---

* Client CA certificate
* List of allowed names (CNs)
* Header names for username, group and extra info
* Check that the TLS connection was authenticated using a client certificate which:
* Was signed by the CA whose certificate matches the retrieved CA certificate.
* Has a CN in the list of allowed CNs, unless the list is blank, in which case all CNs are allowed.