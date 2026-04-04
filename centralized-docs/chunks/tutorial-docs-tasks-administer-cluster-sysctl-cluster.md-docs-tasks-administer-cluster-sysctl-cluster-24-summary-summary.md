---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#24-summary
chunk_level: summary
chunk_type: prose
heading: Setting Sysctls for a Pod
token_count: 103
summary: #### Warning: Only modify sysctl parameters after you understand their effects, to avoid destabilizing your operating system. ``` `apiVersion: v1 kind: Pod metadata: name: sysctl-example spec:...
---

#### Warning:
Only modify sysctl parameters after you understand their effects, to avoid
destabilizing your operating system.
```
`apiVersion: v1
kind: Pod
metadata:
name: sysctl-example
spec:
securityContext:
sysctls:
- name: kernel.shm\_rmid\_forced
value: "0"
- name: net.core.somaxconn
value: "1024"
- name: kernel.msgmax
value: "65536"
...
`
```