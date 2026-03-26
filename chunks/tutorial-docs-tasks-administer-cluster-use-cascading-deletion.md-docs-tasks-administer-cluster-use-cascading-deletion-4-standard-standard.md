---
doc_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion
chunk_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion#4-standard
chunk_level: standard
chunk_type: prose
heading: What's next
token_count: 444
summary: ## Delete owner objects and orphan dependents By default, when you tell Kubernetes to delete an object, the [controller](/docs/concepts/architecture/controller/) also deletes dependent objects. You...
---

## Delete owner objects and orphan dependents
By default, when you tell Kubernetes to delete an object, the
[controller](/docs/concepts/architecture/controller/) also deletes
dependent objects. You can make Kubernetes *orphan* these dependents using
`kubectl` or the Kubernetes API, depending on the Kubernetes version your
cluster runs.
To check the version, enter `kubectl version`.
**Using kubectl**
Run the following command:
```
`kubectl delete deployment nginx-deployment --cascade=orphan
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
-d '{"kind":"DeleteOptions","apiVersion":"v1","propagationPolicy":"Orphan"}' \\
-H "Content-Type: application/json"
`
```
The output contains `orphan` in the `finalizers` field, similar to this:
```
`"kind": "Deployment",
"apiVersion": "apps/v1",
"namespace": "default",
"uid": "6f577034-42a0-479d-be21-78018c466f1f",
"creationTimestamp": "2021-07-09T16:46:37Z",
"deletionTimestamp": "2021-07-09T16:47:08Z",
"deletionGracePeriodSeconds": 0,
"finalizers": [
"orphan"
],
...
`
```
You can check that the Pods managed by the Deployment are still running:
```
`kubectl get pods -l app=nginx
`
```
## What's next
* Learn about [owners and dependents](/docs/concepts/overview/working-with-objects/owners-dependents/) in Kubernetes.
* Learn about Kubernetes [finalizers](/docs/concepts/overview/working-with-objects/finalizers/).
* Learn about [garbage collection](/docs/concepts/architecture/garbage-collection/).