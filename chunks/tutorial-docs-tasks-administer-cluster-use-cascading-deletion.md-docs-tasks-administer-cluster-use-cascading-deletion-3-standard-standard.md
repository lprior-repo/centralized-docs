---
doc_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion
chunk_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion#3-standard
chunk_level: standard
chunk_type: prose
heading: Use background cascading deletion
token_count: 337
summary: ## Use background cascading deletion 1. [Create a sample Deployment](/docs/tasks/run-application/run-stateless-application-deployment/#creating-and-exploring-an-nginx-deployment). 2. Use either...
---

## Use background cascading deletion
1. [Create a sample Deployment](/docs/tasks/run-application/run-stateless-application-deployment/#creating-and-exploring-an-nginx-deployment).
2. Use either `kubectl` or the Kubernetes API to delete the Deployment,
depending on the Kubernetes version your cluster runs.
To check the version, enter `kubectl version`.
You can delete objects using background cascading deletion using `kubectl`
or the Kubernetes API.
Kubernetes uses background cascading deletion by default, and does so
even if you run the following commands without the `--cascade` flag or the
`propagationPolicy` argument.
**Using kubectl**
Run the following command:
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
```
`"kind": "Status",
"apiVersion": "v1",
...
"status": "Success",
"details": {
"name": "nginx-deployment",
"group": "apps",
"kind": "deployments",
"uid": "cc9eefb9-2d49-4445-b1c1-d261c9396456"
}
`
```