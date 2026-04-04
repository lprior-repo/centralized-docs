---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Before you begin
token_count: 330
summary: # Managing Service Accounts A *ServiceAccount* provides an identity for processes that run in a Pod. A process inside a Pod can use the identity of its associated service account to authenticate to...
---

# Managing Service Accounts
A *ServiceAccount* provides an identity for processes that run in a Pod.
A process inside a Pod can use the identity of its associated service account to
authenticate to the cluster's API server.
For an introduction to service accounts, read [configure service accounts](/docs/tasks/configure-pod-container/configure-service-account/).
This task guide explains some of the concepts behind ServiceAccounts. The
guide also explains how to obtain or revoke tokens that represent
ServiceAccounts, and how to (optionally) bind a ServiceAccount's validity to
the lifetime of an API object.
## Before you begin
You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:
* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)
To be able to follow these steps exactly, ensure you have a namespace named
`examplens`.
If you don't, create one by running:
```
`kubectl create namespace examplens
`
```