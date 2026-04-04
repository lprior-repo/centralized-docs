---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#16-summary
chunk_level: summary
chunk_type: prose
heading: Certificate signing requests
token_count: 75
summary: ``` `apiVersion: rbac.authorization.k8s.io/v1 kind: ClusterRole metadata: name: csr-creator rules: - apiGroups: - certificates.k8s.io resources: - certificatesigningrequests verbs: - create - get -...
---

```
`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: csr-creator
rules:
- apiGroups:
- certificates.k8s.io
resources:
- certificatesigningrequests
verbs:
- create
- get
- list
- watch
`
```
To allow approving a CertificateSigningRequest: