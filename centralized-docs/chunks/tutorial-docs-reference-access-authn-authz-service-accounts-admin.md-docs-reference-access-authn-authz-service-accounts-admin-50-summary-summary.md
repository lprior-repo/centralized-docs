---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#50-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 121
summary: ``` `apiVersion: v1 kind: Secret metadata: name: build-robot-secret namespace: default labels: kubernetes.io/legacy-token-last-used: 2022-10-24 kubernetes.io/legacy-token-invalid-since: 2023-10-25...
---

```
`apiVersion: v1
kind: Secret
metadata:
name: build-robot-secret
namespace: default
labels:
kubernetes.io/legacy-token-last-used: 2022-10-24
kubernetes.io/legacy-token-invalid-since: 2023-10-25
annotations:
kubernetes.io/service-account.name: build-robot
type: kubernetes.io/service-account-token
`
```
### ServiceAccount controller
A ServiceAccount controller manages the ServiceAccounts inside namespaces, and
ensures a ServiceAccount named "default" exists in every active namespace.