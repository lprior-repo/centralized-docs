---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#28-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 112
summary: #### Python client To use [Python client](https://github.com/kubernetes-client/python), run the following command: `pip install kubernetes`. See [Python Client Library...
---

#### Python client
To use [Python client](https://github.com/kubernetes-client/python), run the following command:
`pip install kubernetes`. See [Python Client Library page](https://github.com/kubernetes-client/python)
for more installation options.
The Python client can use the same [kubeconfig file](/docs/concepts/configuration/organize-cluster-access-kubeconfig/)
as the kubectl CLI does to locate and authenticate to the API server. See this
[example](https://github.com/kubernetes-client/python/blob/master/examples/out_of_cluster_config.py):