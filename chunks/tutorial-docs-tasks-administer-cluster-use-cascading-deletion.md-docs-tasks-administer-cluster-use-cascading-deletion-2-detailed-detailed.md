---
doc_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion
chunk_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion#2-detailed
chunk_level: detailed
chunk_type: code
heading: Feedback
token_count: 980
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
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified January 11, 2023 at 11:12 AM PST: [Update page weights in /tasks/administer-cluster section (b1202c78ff)](https://github.com/kubernetes/website/commit/b1202c78ff58867d67c2fb13f1c13e37d8857a28)