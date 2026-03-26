---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#20-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 127
summary: with the `--authorization-config` flag set to a file that includes the `RBAC` authorizer; for example: ``` `apiVersion: apiserver.config.k8s.io/v1 kind: AuthorizationConfiguration authorizers: ... -...
---

with the `--authorization-config` flag set to a file that includes the `RBAC` authorizer; for example:
```
`apiVersion: apiserver.config.k8s.io/v1
kind: AuthorizationConfiguration
authorizers:
...
- type: RBAC
...
`
```
Or, start the [API server](/docs/concepts/architecture/#kube-apiserver) with
the `--authorization-mode` flag set to a comma-separated list that includes `RBAC`;
for example:
```
`kube-apiserver --authorization-mode=...,RBAC --other-options --more-options
`
```