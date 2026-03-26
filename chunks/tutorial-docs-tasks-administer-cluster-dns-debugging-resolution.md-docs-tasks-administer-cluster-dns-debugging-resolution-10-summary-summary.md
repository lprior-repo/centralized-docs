---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#10-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 69
summary: ``` `apiVersion: v1 kind: Pod metadata: name: dnsutils namespace: default spec: containers: - name: dnsutils image: registry.k8s.io/e2e-test-images/agnhost:2.39 imagePullPolicy: IfNotPresent...
---

```
`apiVersion: v1
kind: Pod
metadata:
name: dnsutils
namespace: default
spec:
containers:
- name: dnsutils
image: registry.k8s.io/e2e-test-images/agnhost:2.39
imagePullPolicy: IfNotPresent
restartPolicy: Always
`
```