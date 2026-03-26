---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#19-summary
chunk_level: summary
chunk_type: prose
heading: Create a Pod that gets assigned a QoS class of Burstable
token_count: 111
summary: ``` `apiVersion: v1 kind: Pod metadata: name: qos-demo-2 namespace: qos-example spec: containers: - name: qos-demo-2-ctr image: nginx resources: limits: memory: \"200Mi\" requests: memory: \"100Mi\" `...
---

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