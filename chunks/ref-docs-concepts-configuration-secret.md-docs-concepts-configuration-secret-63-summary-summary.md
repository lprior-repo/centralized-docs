---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#63-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 73
summary: ``` `apiVersion: v1 kind: Secret metadata: name: secret-ssh-auth type: kubernetes.io/ssh-auth data: # the data is abbreviated in this example ssh-privatekey: | UG91cmluZzYlRW1vdGljb24lU2N1YmE= ` ```
---

```
`apiVersion: v1
kind: Secret
metadata:
name: secret-ssh-auth
type: kubernetes.io/ssh-auth
data:
# the data is abbreviated in this example
ssh-privatekey: |
UG91cmluZzYlRW1vdGljb24lU2N1YmE= `
```