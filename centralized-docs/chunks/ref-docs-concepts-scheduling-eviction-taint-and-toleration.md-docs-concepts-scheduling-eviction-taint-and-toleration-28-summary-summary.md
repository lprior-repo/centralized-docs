---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#28-summary
chunk_level: summary
chunk_type: prose
heading: Numeric comparison operators
token_count: 87
summary: ``` `apiVersion: v1 kind: Pod metadata: name: nginx-numeric-toleration labels: env: test spec: containers: - name: nginx image: nginx imagePullPolicy: IfNotPresent tolerations: - key:...
---

```
`apiVersion: v1
kind: Pod
metadata:
name: nginx-numeric-toleration
labels:
env: test
spec:
containers:
- name: nginx
image: nginx
imagePullPolicy: IfNotPresent
tolerations:
- key: "servicelevel.organization.example/agreed-service-level"
operator: "Gt"
value: "900"
effect: "NoSchedule"
`
```