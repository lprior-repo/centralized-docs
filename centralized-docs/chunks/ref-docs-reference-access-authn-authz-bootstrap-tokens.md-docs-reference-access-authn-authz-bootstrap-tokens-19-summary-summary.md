---
doc_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens
chunk_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens#19-summary
chunk_level: summary
chunk_type: prose
heading: ConfigMap Signing
token_count: 115
summary: `apiVersion: v1 kind: ConfigMap metadata: name: cluster-info namespace: kube-public data: jws-kubeconfig-07401b: eyJhbGciOiJIUzI1NiIsImtpZCI6IjA3NDAxYiJ9..tYEfbo6zDNo40MQE07aZcQX2m3EB2rO3NuXtxVMYm9U...
---

`apiVersion: v1
kind: ConfigMap
metadata:
name: cluster-info
namespace: kube-public
data:
jws-kubeconfig-07401b: eyJhbGciOiJIUzI1NiIsImtpZCI6IjA3NDAxYiJ9..tYEfbo6zDNo40MQE07aZcQX2m3EB2rO3NuXtxVMYm9U
kubeconfig: |
apiVersion: v1
clusters:
- cluster: