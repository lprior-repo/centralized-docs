---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#46-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 48
summary: ``` `apiVersion: v1 kind: ServiceAccount metadata: name: build-robot namespace: default secrets: - name: build-robot-secret # usually NOT present for a manually generated token ` ```
---

```
`apiVersion: v1
kind: ServiceAccount
metadata:
name: build-robot
namespace: default
secrets:
- name: build-robot-secret # usually NOT present for a manually generated token
`
```