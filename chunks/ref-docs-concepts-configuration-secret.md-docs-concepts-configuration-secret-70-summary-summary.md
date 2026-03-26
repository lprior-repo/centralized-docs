---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#70-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 91
summary: ``` `apiVersion: v1 kind: Secret metadata: name: secret-tls type: kubernetes.io/tls data: # values are base64 encoded, which obscures them but does NOT provide # Replace the following values with...
---

```
`apiVersion: v1
kind: Secret
metadata:
name: secret-tls
type: kubernetes.io/tls
data:
# values are base64 encoded, which obscures them but does NOT provide
# Replace the following values with your own base64-encoded certificate and key.
tls.crt: "REPLACE\_WITH\_BASE64\_CERT"
tls.key: "REPLACE\_WITH\_BASE64\_KEY"`
```