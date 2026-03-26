---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#1-standard
chunk_level: standard
chunk_type: prose
heading: Create a namespace
token_count: 441
summary: # Configure Quality of Service for Pods This page shows how to configure Pods so that they will be assigned particular [Quality of Service (QoS) classes](/docs/concepts/workloads/pods/pod-qos/)....
---

# Configure Quality of Service for Pods
This page shows how to configure Pods so that they will be assigned particular
[Quality of Service (QoS) classes](/docs/concepts/workloads/pods/pod-qos/).
Kubernetes uses QoS classes to make decisions about evicting Pods when Node resources are exceeded.
When Kubernetes creates a Pod it assigns one of these QoS classes to the Pod:
* [Guaranteed](/docs/concepts/workloads/pods/pod-qos/#guaranteed)
* [Burstable](/docs/concepts/workloads/pods/pod-qos/#burstable)
* [BestEffort](/docs/concepts/workloads/pods/pod-qos/#besteffort)
#### Note:
Kubernetes assigns the QoS class when the Pod is created, and it remains unchanged
for the lifetime of the Pod. If you attempt to
[resize the Pod's resources](/docs/tasks/configure-pod-container/resize-container-resources/)
to values that would result in a different QoS class, control plane rejects your request with an error message.
## Before you begin
You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:
* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)
You also need to be able to create and delete namespaces.
## Create a namespace
Create a namespace so that the resources you create in this exercise are
isolated from the rest of your cluster.
```
`kubectl create namespace qos-example
`
```