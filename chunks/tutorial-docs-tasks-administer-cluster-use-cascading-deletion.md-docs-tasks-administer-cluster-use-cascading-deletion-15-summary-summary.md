---
doc_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion
chunk_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion#15-summary
chunk_level: summary
chunk_type: prose
heading: Use background cascading deletion
token_count: 120
summary: ``` `kubectl delete deployment nginx-deployment --cascade=background ` ``` **Using the Kubernetes API** 1. Start a local proxy session: ``` `kubectl proxy --port=8080 ` ``` 2. Use `curl` to trigger...
---

```
`kubectl delete deployment nginx-deployment --cascade=background
`
```
**Using the Kubernetes API**
1. Start a local proxy session:
```
`kubectl proxy --port=8080
`
```
2. Use `curl` to trigger deletion:
```
`curl -X DELETE localhost:8080/apis/apps/v1/namespaces/default/deployments/nginx-deployment \\
-d '{"kind":"DeleteOptions","apiVersion":"v1","propagationPolicy":"Background"}' \\
-H "Content-Type: application/json"
`
```
The output is similar to this: