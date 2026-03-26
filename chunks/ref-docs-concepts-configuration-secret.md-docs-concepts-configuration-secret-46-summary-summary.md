---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#46-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 57
summary: ``` `apiVersion: v1 kind: Secret metadata: name: secret-sa-sample annotations: kubernetes.io/service-account.name: \"sa-name\" type: kubernetes.io/service-account-token data: extra: YmFyCg==` ```
---

```
`apiVersion: v1
kind: Secret
metadata:
name: secret-sa-sample
annotations:
kubernetes.io/service-account.name: "sa-name"
type: kubernetes.io/service-account-token
data:
extra: YmFyCg==`
```