---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#56-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 89
summary: ``` `apiVersion: rbac.authorization.k8s.io/v1 kind: Role metadata: namespace: default name: configmap-updater rules: - apiGroups: [\"\"] # at the HTTP level, the name of the resource for accessing...
---

```
`apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
namespace: default
name: configmap-updater
rules:
- apiGroups: [""]
# at the HTTP level, the name of the resource for accessing ConfigMap
# objects is "configmaps"
resources: ["configmaps"]
resourceNames: ["my-configmap"]
verbs: ["update", "get"]
`
```