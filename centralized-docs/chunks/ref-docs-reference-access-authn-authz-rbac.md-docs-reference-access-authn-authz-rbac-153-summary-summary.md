---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#153-summary
chunk_level: summary
chunk_type: prose
heading: Write access for EndpointSlices
token_count: 121
summary: be accessible, and can circumvent network policies or security controls intended to prevent/isolate access to those backends. EndpointSlices were never included in the edit or admin roles, so there...
---

be accessible, and can circumvent network policies or security controls
intended to prevent/isolate access to those backends.
EndpointSlices were never included in the edit or admin roles, so there
is nothing to restore for the EndpointSlice API.
labels:
rbac.authorization.k8s.io/aggregate-to-edit: "true"
name: custom:aggregate-to-edit:endpoints # you can change this if you wish
rules:
- apiGroups: [""]
resources: ["endpoints"]
verbs: ["create", "delete", "deletecollection", "patch", "update"]
`
```