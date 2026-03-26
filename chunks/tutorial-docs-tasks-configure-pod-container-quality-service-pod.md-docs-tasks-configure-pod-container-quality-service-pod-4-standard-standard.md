---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#4-standard
chunk_level: standard
chunk_type: prose
heading: Create a Pod that gets assigned a QoS class of BestEffort
token_count: 319
summary: ## Create a Pod that gets assigned a QoS class of BestEffort For a Pod to be given a QoS class of `BestEffort`, the Containers in the Pod must not have any memory or CPU limits or requests. Here is a...
---

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