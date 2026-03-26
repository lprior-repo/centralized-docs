---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#2-detailed
chunk_level: detailed
chunk_type: code
heading: Create a Pod that gets assigned a QoS class of BestEffort
token_count: 714
summary: ## Create a Pod that gets assigned a QoS class of Burstable A Pod is given a QoS class of `Burstable` if: * The Pod does not meet the criteria for QoS class `Guaranteed`. * At least one Container in...
---

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