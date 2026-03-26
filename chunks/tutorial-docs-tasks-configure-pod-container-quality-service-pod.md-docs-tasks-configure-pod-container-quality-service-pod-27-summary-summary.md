---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#27-summary
chunk_level: summary
chunk_type: prose
heading: Create a Pod that has two Containers
token_count: 124
summary: ``` `apiVersion: v1 kind: Pod metadata: name: qos-demo-4 namespace: qos-example spec: containers: - name: qos-demo-4-ctr-1 image: nginx resources: requests: memory: \"200Mi\" - name: qos-demo-4-ctr-2...
---

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