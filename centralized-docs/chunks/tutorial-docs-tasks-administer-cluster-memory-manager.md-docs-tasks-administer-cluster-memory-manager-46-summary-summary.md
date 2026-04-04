---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#46-summary
chunk_level: summary
chunk_type: prose
heading: Reserved memory configuration
token_count: 115
summary: Also, a pod sharing CPU(s) runs in the `Guaranteed` QoS class, when `requests` are equal to `limits`. ``` `spec: containers: - name: nginx image: nginx resources: limits: memory: \"200Mi\" cpu: \"300m\"...
---

Also, a pod sharing CPU(s) runs in the `Guaranteed` QoS class, when `requests` are equal to `limits`.
```
`spec:
containers:
- name: nginx
image: nginx
resources:
limits:
memory: "200Mi"
cpu: "300m"
example.com/device: "1"
requests:
memory: "200Mi"
cpu: "300m"
example.com/device: "1"
`
```
Notice that both CPU and memory requests must be specified for a Pod to lend it to Guaranteed QoS class.