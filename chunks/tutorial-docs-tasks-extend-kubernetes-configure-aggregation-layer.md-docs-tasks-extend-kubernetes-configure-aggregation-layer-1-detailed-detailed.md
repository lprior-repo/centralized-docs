---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Before you begin
token_count: 343
summary: # Configure the Aggregation Layer Configuring the [aggregation layer](/docs/concepts/extend-kubernetes/api-extension/apiserver-aggregation/) allows the Kubernetes apiserver to be extended with...
---

# Configure the Aggregation Layer
Configuring the [aggregation layer](/docs/concepts/extend-kubernetes/api-extension/apiserver-aggregation/)
allows the Kubernetes apiserver to be extended with additional APIs, which are not
part of the core Kubernetes APIs.
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
#### Note:
There are a few setup requirements for getting the aggregation layer working in
your environment to support mutual TLS auth between the proxy and extension apiservers.
Kubernetes and the kube-apiserver have multiple CAs, so make sure that the proxy is
signed by the aggregation layer CA and not by something else, like the Kubernetes general CA.
#### Caution:
Reusing the same CA for different client types can negatively impact the cluster's
ability to function. For more information, see [CA Reusage and Conflicts](#ca-reusage-and-conflicts).