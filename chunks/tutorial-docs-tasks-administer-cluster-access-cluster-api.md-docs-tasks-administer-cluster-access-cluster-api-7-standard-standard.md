---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#7-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 200
summary: #### Python client To use [Python client](https://github.com/kubernetes-client/python), run the following command: `pip install kubernetes`. See [Python Client Library...
---

#### Python client
To use [Python client](https://github.com/kubernetes-client/python), run the following command:
`pip install kubernetes`. See [Python Client Library page](https://github.com/kubernetes-client/python)
for more installation options.
The Python client can use the same [kubeconfig file](/docs/concepts/configuration/organize-cluster-access-kubeconfig/)
as the kubectl CLI does to locate and authenticate to the API server. See this
[example](https://github.com/kubernetes-client/python/blob/master/examples/out_of_cluster_config.py):
```
`from kubernetes import client, config
config.load\_kube\_config()
v1=client.CoreV1Api()
print("Listing pods with their IPs:")
ret = v1.list\_pod\_for\_all\_namespaces(watch=False)
for i in ret.items:
print("%s\\t%s\\t%s" % (i.status.pod\_ip, i.metadata.namespace, i.metadata.name))
`
```