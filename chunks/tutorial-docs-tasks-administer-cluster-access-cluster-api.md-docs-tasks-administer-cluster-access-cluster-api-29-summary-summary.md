---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#29-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 88
summary: ``` `from kubernetes import client, config config.load\_kube\_config() v1=client.CoreV1Api() print(\"Listing pods with their IPs:\") ret = v1.list\_pod\_for\_all\_namespaces(watch=False) for i in...
---

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