---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#2-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 332
summary: ## Before you begin You need to have a Kubernetes cluster, and the kubectl command-line tool must be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster...
---

## Before you begin
You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:
* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)
To check the version, enter `kubectl version`.
### Accessing for the first time with kubectl
When accessing the Kubernetes API for the first time, use the
Kubernetes command-line tool, `kubectl`.
To access a cluster, you need to know the location of the cluster and have credentials
to access it. Typically, this is automatically set-up when you work through
a [Getting started guide](/docs/setup/),
or someone else set up the cluster and provided you with credentials and a location.
Check the location and credentials that kubectl knows about with this command:
```
`kubectl config view
`
```
Many of the [examples](https://github.com/kubernetes/examples/tree/master/) provide an introduction to using
kubectl. Complete documentation is found in the [kubectl manual](/docs/reference/kubectl/).