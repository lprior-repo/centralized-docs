---
doc_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion
chunk_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion#1-standard
chunk_level: standard
chunk_type: prose
heading: Check owner references on your pods
token_count: 424
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