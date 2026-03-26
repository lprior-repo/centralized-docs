---
doc_id: tutorial/docs-concepts-extend-kubernetes-api-extension.md/docs-concepts-extend-kubernetes-api-extension
chunk_id: tutorial/docs-concepts-extend-kubernetes-api-extension.md/docs-concepts-extend-kubernetes-api-extension#3-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 97
summary: * The [aggregation layer](/docs/concepts/extend-kubernetes/api-extension/apiserver-aggregation/) sits behind the primary API server, which acts as a proxy. This arrangement is called API Aggregation...
---

* The [aggregation layer](/docs/concepts/extend-kubernetes/api-extension/apiserver-aggregation/)
sits behind the primary API server, which acts as a proxy.
This arrangement is called API Aggregation (AA), which allows you to provide
specialized implementations for your custom resources by writing and
deploying your own API server.
The main API server delegates requests to your API server for the custom APIs that you specify,
making them available to all of its clients.