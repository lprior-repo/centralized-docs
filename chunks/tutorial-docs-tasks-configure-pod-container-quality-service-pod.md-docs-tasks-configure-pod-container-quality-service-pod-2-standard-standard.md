---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#2-standard
chunk_level: standard
chunk_type: prose
heading: Create a Pod that gets assigned a QoS class of Guaranteed
token_count: 482
summary: ## Create a Pod that gets assigned a QoS class of Guaranteed For a Pod to be given a QoS class of `Guaranteed`: * Every Container in the Pod must have a memory limit and a memory request. * For every...
---

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