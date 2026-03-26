---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#24-summary
chunk_level: summary
chunk_type: prose
heading: Create a Pod that gets assigned a QoS class of BestEffort
token_count: 115
summary: ``` `apiVersion: v1 kind: Pod metadata: name: qos-demo-3 namespace: qos-example spec: containers: - name: qos-demo-3-ctr image: nginx ` ``` Create the Pod: ``` `kubectl apply -f...
---

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