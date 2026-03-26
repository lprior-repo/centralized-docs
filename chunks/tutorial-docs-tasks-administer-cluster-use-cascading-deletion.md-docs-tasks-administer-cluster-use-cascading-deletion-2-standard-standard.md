---
doc_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion
chunk_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion#2-standard
chunk_level: standard
chunk_type: prose
heading: Use foreground cascading deletion
token_count: 380
summary: ## Use foreground cascading deletion By default, Kubernetes uses [background cascading deletion](/docs/concepts/architecture/garbage-collection/#background-deletion) to delete dependents of an...
---

## Use foreground cascading deletion
By default, Kubernetes uses [background cascading deletion](/docs/concepts/architecture/garbage-collection/#background-deletion)
to delete dependents of an object. You can switch to foreground cascading deletion
using either `kubectl` or the Kubernetes API, depending on the Kubernetes
version your cluster runs.
To check the version, enter `kubectl version`.
You can delete objects using foreground cascading deletion using `kubectl` or the
Kubernetes API.
**Using kubectl**
Run the following command:
```
`kubectl delete deployment nginx-deployment --cascade=foreground
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
-d '{"kind":"DeleteOptions","apiVersion":"v1","propagationPolicy":"Foreground"}' \\
-H "Content-Type: application/json"
`
```
The output contains a `foregroundDeletion` [finalizer](/docs/concepts/overview/working-with-objects/finalizers/)
like this:
```
`"kind": "Deployment",
"apiVersion": "apps/v1",
"metadata": {
"name": "nginx-deployment",
"namespace": "default",
"uid": "d1ce1b02-cae8-4288-8a53-30e84d8fa505",
"resourceVersion": "1363097",
"creationTimestamp": "2021-07-08T20:24:37Z",
"deletionTimestamp": "2021-07-08T20:27:39Z",
"finalizers": [
"foregroundDeletion"
]
...
`
```