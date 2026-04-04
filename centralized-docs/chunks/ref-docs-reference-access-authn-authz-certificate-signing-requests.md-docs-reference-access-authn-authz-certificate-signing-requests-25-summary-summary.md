---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#25-summary
chunk_level: summary
chunk_type: prose
heading: Certificate signing requests
token_count: 104
summary: `apiVersion: rbac.authorization.k8s.io/v1 kind: ClusterRole metadata: name: csr-signer rules: - apiGroups: - certificates.k8s.io resources: - certificatesigningrequests verbs: - get - list - watch -...
---

`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: csr-signer
rules:
- apiGroups:
- certificates.k8s.io
resources:
- certificatesigningrequests
verbs:
- get
- list
- watch
- apiGroups:
- certificates.k8s.io
resources:
- certificatesigningrequests/status
verbs:
- update
- apiGroups:
- certificates.k8s.io
resources:
- signers
resourceNames: