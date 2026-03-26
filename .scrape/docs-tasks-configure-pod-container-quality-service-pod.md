---
url: https://kubernetes.io/docs/tasks/configure-pod-container/quality-service-pod/
title: Configure Quality of Service for Pods
word_count: 1279
filtered: true
elements_removed: 0
density_score: 0.93
---

## Table of Contents

- [Configure Quality of Service for Pods](#configure-quality-of-service-for-pods)
      - [Note:](#note)
  - [Before you begin](#before-you-begin)
  - [Create a namespace](#create-a-namespace)
  - [Create a Pod that gets assigned a QoS class of Guaranteed](#create-a-pod-that-gets-assigned-a-qos-class-of-guaranteed)
      - [Note:](#note)
      - [Clean up](#clean-up)
  - [Create a Pod that gets assigned a QoS class of Burstable](#create-a-pod-that-gets-assigned-a-qos-class-of-burstable)
      - [Clean up](#clean-up)
  - [Create a Pod that gets assigned a QoS class of BestEffort](#create-a-pod-that-gets-assigned-a-qos-class-of-besteffort)
      - [Clean up](#clean-up)
  - [Create a Pod that has two Containers](#create-a-pod-that-has-two-containers)
  - [Retrieve the QoS class for a Pod](#retrieve-the-qos-class-for-a-pod)
  - [Clean up](#clean-up)
    - [For app developers](#for-app-developers)
    - [For cluster administrators](#for-cluster-administrators)
  - [Feedback](#feedback)

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
## Create a Pod that gets assigned a QoS class of Burstable
A Pod is given a QoS class of `Burstable` if:
* The Pod does not meet the criteria for QoS class `Guaranteed`.
* At least one Container in the Pod has a memory or CPU request or limit.
Here is a manifest for a Pod that has one Container. The Container has a memory limit of 200 MiB
and a memory request of 100 MiB.
[`pods/qos/qos-pod-2.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/qos/qos-pod-2.yaml)![](/images/copycode.svg "Copy pods/qos/qos-pod-2.yaml to clipboard")
```
`apiVersion: v1
kind: Pod
metadata:
name: qos-demo-2
namespace: qos-example
spec:
containers:
- name: qos-demo-2-ctr
image: nginx
resources:
limits:
memory: "200Mi"
requests:
memory: "100Mi"
`
```
Create the Pod:
```
`kubectl apply -f https://k8s.io/examples/pods/qos/qos-pod-2.yaml --namespace=qos-example
`
```
View detailed information about the Pod:
```
`kubectl get pod qos-demo-2 --namespace=qos-example --output=yaml
`
```
The output shows that Kubernetes gave the Pod a QoS class of `Burstable`:
```
`spec:
containers:
- image: nginx
imagePullPolicy: Always
name: qos-demo-2-ctr
resources:
limits:
memory: 200Mi
requests:
memory: 100Mi
...
status:
qosClass: Burstable
`
```
#### Clean up
Delete your Pod:
```
`kubectl delete pod qos-demo-2 --namespace=qos-example
`
```
## Create a Pod that gets assigned a QoS class of BestEffort
For a Pod to be given a QoS class of `BestEffort`, the Containers in the Pod must not
have any memory or CPU limits or requests.
Here is a manifest for a Pod that has one Container. The Container has no memory or CPU
limits or requests:
[`pods/qos/qos-pod-3.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/qos/qos-pod-3.yaml)![](/images/copycode.svg "Copy pods/qos/qos-pod-3.yaml to clipboard")
```
`apiVersion: v1
kind: Pod
metadata:
name: qos-demo-3
namespace: qos-example
spec:
containers:
- name: qos-demo-3-ctr
image: nginx
`
```
Create the Pod:
```
`kubectl apply -f https://k8s.io/examples/pods/qos/qos-pod-3.yaml --namespace=qos-example
`
```
View detailed information about the Pod:
```
`kubectl get pod qos-demo-3 --namespace=qos-example --output=yaml
`
```
The output shows that Kubernetes gave the Pod a QoS class of `BestEffort`:
```
`spec:
containers:
...
resources: {}
...
status:
qosClass: BestEffort
`
```
#### Clean up
Delete your Pod:
```
`kubectl delete pod qos-demo-3 --namespace=qos-example
`
```
## Create a Pod that has two Containers
Here is a manifest for a Pod that has two Containers. One container specifies a memory
request of 200 MiB. The other Container does not specify any requests or limits.
[`pods/qos/qos-pod-4.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/qos/qos-pod-4.yaml)![](/images/copycode.svg "Copy pods/qos/qos-pod-4.yaml to clipboard")
```
`apiVersion: v1
kind: Pod
metadata:
name: qos-demo-4
namespace: qos-example
spec:
containers:
- name: qos-demo-4-ctr-1
image: nginx
resources:
requests:
memory: "200Mi"
- name: qos-demo-4-ctr-2
image: redis
`
```
Notice that this Pod meets the criteria for QoS class `Burstable`. That is, it does not meet the
criteria for QoS class `Guaranteed`, and one of its Containers has a memory request.
Create the Pod:
```
`kubectl apply -f https://k8s.io/examples/pods/qos/qos-pod-4.yaml --namespace=qos-example
`
```
View detailed information about the Pod:
```
`kubectl get pod qos-demo-4 --namespace=qos-example --output=yaml
`
```
The output shows that Kubernetes gave the Pod a QoS class of `Burstable`:
```
`spec:
containers:
...
name: qos-demo-4-ctr-1
resources:
requests:
memory: 200Mi
...
name: qos-demo-4-ctr-2
resources: {}
...
status:
qosClass: Burstable
`
```
## Retrieve the QoS class for a Pod
Rather than see all the fields, you can view just the field you need:
```
`kubectl --namespace=qos-example get pod qos-demo-4 -o jsonpath='{ .status.qosClass}{"\\n"}'
`
```
```
`Burstable
`
```
## Clean up
Delete your namespace:
```
`kubectl delete namespace qos-example
`
```
### For app developers
* [Assign Memory Resources to Containers and Pods](/docs/tasks/configure-pod-container/assign-memory-resource/)
* [Assign CPU Resources to Containers and Pods](/docs/tasks/configure-pod-container/assign-cpu-resource/)
### For cluster administrators
* [Configure Default Memory Requests and Limits for a Namespace](/docs/tasks/administer-cluster/manage-resources/memory-default-namespace/)
* [Configure Default CPU Requests and Limits for a Namespace](/docs/tasks/administer-cluster/manage-resources/cpu-default-namespace/)
* [Configure Minimum and Maximum Memory Constraints for a Namespace](/docs/tasks/administer-cluster/manage-resources/memory-constraint-namespace/)
* [Configure Minimum and Maximum CPU Constraints for a Namespace](/docs/tasks/administer-cluster/manage-resources/cpu-constraint-namespace/)
* [Configure Memory and CPU Quotas for a Namespace](/docs/tasks/administer-cluster/manage-resources/quota-memory-cpu-namespace/)
* [Configure a Pod Quota for a Namespace](/docs/tasks/administer-cluster/manage-resources/quota-pod-namespace/)
* [Configure Quotas for API Objects](/docs/tasks/administer-cluster/quota-api-object/)
* [Control Topology Management policies on a node](/docs/tasks/administer-cluster/topology-manager/)
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
Last modified January 10, 2026 at 2:42 PM PST: [revamp the doc as per the changes required (48b81e1260)](https://github.com/kubernetes/website/commit/48b81e12607307aebc1ce27db824756016fa4c58)
## Related Pages

- [Deploy and Access the Kubernetes Dashboard](docs-tasks-access-application-cluster-web-ui-dashboard.md)
- [Pod Priority and Preemption](docs-concepts-scheduling-eviction-pod-priority-preemption.md)
- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
