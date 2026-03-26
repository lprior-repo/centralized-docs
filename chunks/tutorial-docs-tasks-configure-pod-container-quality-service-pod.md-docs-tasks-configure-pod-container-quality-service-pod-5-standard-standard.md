---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#5-standard
chunk_level: standard
chunk_type: code
heading: Retrieve the QoS class for a Pod
token_count: 436
summary: ## Create a Pod that has two Containers Here is a manifest for a Pod that has two Containers. One container specifies a memory request of 200 MiB. The other Container does not specify any requests or...
---

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