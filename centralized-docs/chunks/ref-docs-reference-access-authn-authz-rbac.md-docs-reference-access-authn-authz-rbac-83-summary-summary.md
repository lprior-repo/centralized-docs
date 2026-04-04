---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#83-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 124
summary: For the default service account in the \"kube-system\" namespace: ``` `subjects: - kind: ServiceAccount name: default namespace: kube-system ` ``` For all service accounts in the \"qa\" namespace: ```...
---

For the default service account in the "kube-system" namespace:
```
`subjects:
- kind: ServiceAccount
name: default
namespace: kube-system
`
```
For all service accounts in the "qa" namespace:
```
`subjects:
- kind: Group
name: system:serviceaccounts:qa
apiGroup: rbac.authorization.k8s.io
`
```
For all service accounts in any namespace:
```
`subjects:
- kind: Group
name: system:serviceaccounts
apiGroup: rbac.authorization.k8s.io
`
```
For all authenticated users: