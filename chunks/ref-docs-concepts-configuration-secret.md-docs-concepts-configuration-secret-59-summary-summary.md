---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#59-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 63
summary: ``` `apiVersion: v1 kind: Secret metadata: name: secret-basic-auth type: kubernetes.io/basic-auth stringData: username: admin # required field for kubernetes.io/basic-auth password: t0p-Secret #...
---

```
`apiVersion: v1
kind: Secret
metadata:
name: secret-basic-auth
type: kubernetes.io/basic-auth
stringData:
username: admin # required field for kubernetes.io/basic-auth
password: t0p-Secret # required field for kubernetes.io/basic-auth`
```