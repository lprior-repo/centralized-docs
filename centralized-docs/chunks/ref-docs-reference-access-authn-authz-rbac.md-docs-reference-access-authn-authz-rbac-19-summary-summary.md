---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#19-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 124
summary: Role-based access control (RBAC) is a method of regulating access to computer or network resources based on the roles of individual users within your organization. RBAC authorization uses the...
---

Role-based access control (RBAC) is a method of regulating access to computer or
network resources based on the roles of individual users within your organization.
RBAC authorization uses the `rbac.authorization.k8s.io`
[API group](/docs/concepts/overview/kubernetes-api/#api-groups-and-versioning) to drive authorization
decisions, allowing you to dynamically configure policies through the Kubernetes API.
To enable RBAC, start the [API server](/docs/concepts/architecture/#kube-apiserver)
with the `--authorization-config` flag set to a file that includes the `RBAC`