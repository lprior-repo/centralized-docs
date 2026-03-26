---
doc_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion
chunk_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion#1-detailed
chunk_level: detailed
chunk_type: code
heading: Use foreground cascading deletion
token_count: 805
summary: # Use Cascading Deletion in a Cluster This page shows you how to specify the type of [cascading deletion](/docs/concepts/architecture/garbage-collection/#cascading-deletion) to use in your cluster...
---

# Use Cascading Deletion in a Cluster
This page shows you how to specify the type of
[cascading deletion](/docs/concepts/architecture/garbage-collection/#cascading-deletion)
to use in your cluster during [garbage collection](/docs/concepts/architecture/garbage-collection/).
## Before you begin
You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:
* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)
You also need to [create a sample Deployment](/docs/tasks/run-application/run-stateless-application-deployment/#creating-and-exploring-an-nginx-deployment)
to experiment with the different types of cascading deletion. You will need to
recreate the Deployment for each type.
## Check owner references on your pods
Check that the `ownerReferences` field is present on your pods:
```
`kubectl get pods -l app=nginx --output=yaml
`
```
The output has an `ownerReferences` field similar to this:
```
`apiVersion: v1
...
ownerReferences:
- apiVersion: apps/v1
blockOwnerDeletion: true
controller: true
kind: ReplicaSet
name: nginx-deployment-6b474476c4
uid: 4fdcd81c-bd5d-41f7-97af-3a3b759af9a7
...
`
```
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