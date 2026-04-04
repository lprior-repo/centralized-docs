---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#45-summary
chunk_level: summary
chunk_type: prose
heading: Reserved memory configuration
token_count: 91
summary: ``` `spec: containers: - name: nginx image: nginx resources: limits: memory: \"200Mi\" cpu: \"2\" example.com/device: \"1\" requests: memory: \"200Mi\" cpu: \"2\" example.com/device: \"1\" ` ``` Also, a pod...
---

```
`spec:
containers:
- name: nginx
image: nginx
resources:
limits:
memory: "200Mi"
cpu: "2"
example.com/device: "1"
requests:
memory: "200Mi"
cpu: "2"
example.com/device: "1"
`
```
Also, a pod sharing CPU(s) runs in the `Guaranteed` QoS class, when `requests` are equal to `limits`.