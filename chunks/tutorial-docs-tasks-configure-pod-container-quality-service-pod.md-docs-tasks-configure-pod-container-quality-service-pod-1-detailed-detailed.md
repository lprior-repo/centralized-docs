---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#1-detailed
chunk_level: detailed
chunk_type: code
heading: Create a Pod that gets assigned a QoS class of Guaranteed
token_count: 1014
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
## Create a Pod that gets assigned a QoS class of Guaranteed
For a Pod to be given a QoS class of `Guaranteed`:
* Every Container in the Pod must have a memory limit and a memory request.
* For every Container in the Pod, the memory limit must equal the memory request.
* Every Container in the Pod must have a CPU limit and a CPU request.
* For every Container in the Pod, the CPU limit must equal the CPU request.
These restrictions apply to init containers and app containers equally.
[Ephemeral containers](/docs/concepts/workloads/pods/ephemeral-containers/)
cannot define resources so these restrictions do not apply.
Here is a manifest for a Pod that has one Container. The Container has a memory limit and a
memory request, both equal to 200 MiB. The Container has a CPU limit and a CPU request, both equal to 700 milliCPU:
[`pods/qos/qos-pod.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/qos/qos-pod.yaml)![](/images/copycode.svg "Copy pods/qos/qos-pod.yaml to clipboard")
```
`apiVersion: v1
kind: Pod
metadata:
name: qos-demo
namespace: qos-example
spec:
containers:
- name: qos-demo-ctr
image: nginx
resources:
limits:
memory: "200Mi"
cpu: "700m"
requests:
memory: "200Mi"
cpu: "700m"
`
```
Create the Pod:
```
`kubectl apply -f https://k8s.io/examples/pods/qos/qos-pod.yaml --namespace=qos-example
`
```
View detailed information about the Pod:
```
`kubectl get pod qos-demo --namespace=qos-example --output=yaml
`
```
The output shows that Kubernetes gave the Pod a QoS class of `Guaranteed`. The output also
verifies that the Pod Container has a memory request that matches its memory limit, and it has
a CPU request that matches its CPU limit.
```
`spec:
containers:
...
resources:
limits:
cpu: 700m
memory: 200Mi
requests:
cpu: 700m
memory: 200Mi
...
status:
qosClass: Guaranteed
`
```
#### Note:
If a Container specifies its own memory limit, but does not specify a memory request, Kubernetes
automatically assigns a memory request that matches the limit. Similarly, if a Container specifies its own
CPU limit, but does not specify a CPU request, Kubernetes automatically assigns a CPU request that matches
the limit.
#### Clean up
Delete your Pod:
```
`kubectl delete pod qos-demo --namespace=qos-example
`
```