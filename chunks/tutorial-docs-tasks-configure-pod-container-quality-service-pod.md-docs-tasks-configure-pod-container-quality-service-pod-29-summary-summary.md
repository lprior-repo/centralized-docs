---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#29-summary
chunk_level: summary
chunk_type: prose
heading: Create a Pod that has two Containers
token_count: 74
summary: The output shows that Kubernetes gave the Pod a QoS class of `Burstable`: ``` `spec: containers: ... name: qos-demo-4-ctr-1 resources: requests: memory: 200Mi ... name: qos-demo-4-ctr-2 resources: {}...
---

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