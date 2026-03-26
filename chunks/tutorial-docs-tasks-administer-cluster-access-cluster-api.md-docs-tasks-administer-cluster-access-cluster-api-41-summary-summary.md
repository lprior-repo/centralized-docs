---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#41-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 120
summary: #### JavaScript client To install [JavaScript client](https://github.com/kubernetes-client/javascript), run the following command: `npm install @kubernetes/client-node`. See...
---

#### JavaScript client
To install [JavaScript client](https://github.com/kubernetes-client/javascript),
run the following command: `npm install @kubernetes/client-node`. See
[https://github.com/kubernetes-client/javascript/releases](https://github.com/kubernetes-client/javascript/releases)
to see which versions are supported.
The JavaScript client can use the same [kubeconfig file](/docs/concepts/configuration/organize-cluster-access-kubeconfig/)
as the kubectl CLI does to locate and authenticate to the API server. See this
[example](https://github.com/kubernetes-client/javascript/blob/master/examples/example.js):