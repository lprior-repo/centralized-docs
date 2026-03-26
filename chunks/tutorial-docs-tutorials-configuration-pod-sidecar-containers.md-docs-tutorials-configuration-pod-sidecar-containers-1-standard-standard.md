---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#1-standard
chunk_level: standard
chunk_type: prose
heading: Objectives
token_count: 368
summary: # Adopting Sidecar Containers This section is relevant for people adopting a new built-in [sidecar containers](/docs/concepts/workloads/pods/sidecar-containers/) feature for their workloads. Sidecar...
---

# Adopting Sidecar Containers
This section is relevant for people adopting a new built-in
[sidecar containers](/docs/concepts/workloads/pods/sidecar-containers/) feature for their workloads.
Sidecar container is not a new concept as posted in the
[blog post](/blog/2015/06/the-distributed-system-toolkit-patterns/).
Kubernetes allows running multiple containers in a Pod to implement this concept.
However, running a sidecar container as a regular container
has a lot of limitations being fixed with the new built-in sidecar containers support.
FEATURE STATE:
`Kubernetes v1.33 [stable]`(enabled by default)
## Objectives
* Understand the need for sidecar containers
* Be able to troubleshoot issues with the sidecar containers
* Understand options to universally "inject" sidecar containers to any workload## Before you begin
You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:
* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)Your Kubernetes server must be at or later than version 1.29.
To check the version, enter `kubectl version`.