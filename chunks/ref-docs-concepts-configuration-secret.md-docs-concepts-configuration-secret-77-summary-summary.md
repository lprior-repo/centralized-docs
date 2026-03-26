---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#77-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 120
summary: `apiVersion: v1 kind: Secret metadata: name: bootstrap-token-5emitj namespace: kube-system type: bootstrap.kubernetes.io/token data: auth-extra-groups:...
---

`apiVersion: v1
kind: Secret
metadata:
name: bootstrap-token-5emitj
namespace: kube-system
type: bootstrap.kubernetes.io/token
data:
auth-extra-groups: c3lzdGVtOmJvb3RzdHJhcHBlcnM6a3ViZWFkbTpkZWZhdWx0LW5vZGUtdG9rZW4=
expiration: MjAyMC0wOS0xM1QwNDozOToxMFo=
token-id: NWVtaXRq