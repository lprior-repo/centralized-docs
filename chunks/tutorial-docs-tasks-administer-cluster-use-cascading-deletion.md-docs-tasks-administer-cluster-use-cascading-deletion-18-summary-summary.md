---
doc_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion
chunk_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion#18-summary
chunk_level: summary
chunk_type: prose
heading: Delete owner objects and orphan dependents
token_count: 116
summary: **Using the Kubernetes API** 1. Start a local proxy session: ``` `kubectl proxy --port=8080 ` ``` 2. Use `curl` to trigger deletion: ``` `curl -X DELETE...
---

**Using the Kubernetes API**
1. Start a local proxy session:
```
`kubectl proxy --port=8080
`
```
2. Use `curl` to trigger deletion:
```
`curl -X DELETE localhost:8080/apis/apps/v1/namespaces/default/deployments/nginx-deployment \\
-d '{"kind":"DeleteOptions","apiVersion":"v1","propagationPolicy":"Orphan"}' \\
-H "Content-Type: application/json"
`
```
The output contains `orphan` in the `finalizers` field, similar to this: