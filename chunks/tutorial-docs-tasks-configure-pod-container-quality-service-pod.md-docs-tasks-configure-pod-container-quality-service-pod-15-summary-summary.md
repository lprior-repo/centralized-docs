---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#15-summary
chunk_level: summary
chunk_type: prose
heading: Create a Pod that gets assigned a QoS class of Guaranteed
token_count: 115
summary: ``` `spec: containers: ... resources: limits: cpu: 700m memory: 200Mi requests: cpu: 700m memory: 200Mi ... status: qosClass: Guaranteed ` ``` #### Note: If a Container specifies its own memory...
---

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