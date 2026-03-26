---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#98-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 75
summary: ``` `apiVersion: v1 kind: Pod metadata: name: mypod spec: containers: - name: mypod image: redis volumeMounts: - name: foo mountPath: \"/etc/foo\" readOnly: true volumes: - name: foo secret:...
---

```
`apiVersion: v1
kind: Pod
metadata:
name: mypod
spec:
containers:
- name: mypod
image: redis
volumeMounts:
- name: foo
mountPath: "/etc/foo"
readOnly: true
volumes:
- name: foo
secret:
secretName: mysecret
optional: true`
```