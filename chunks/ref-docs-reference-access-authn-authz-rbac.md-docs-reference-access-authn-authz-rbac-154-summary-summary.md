---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#154-summary
chunk_level: summary
chunk_type: prose
heading: Upgrading from ABAC
token_count: 114
summary: ## Upgrading from ABAC Clusters that originally ran older Kubernetes versions often used permissive ABAC policies, including granting full API access to all service accounts. Default RBAC policies...
---

## Upgrading from ABAC
Clusters that originally ran older Kubernetes versions often used
permissive ABAC policies, including granting full API access to all
service accounts.
Default RBAC policies grant scoped permissions to control-plane components, nodes,
and controllers, but grant *no permissions* to service accounts outside the `kube-system` namespace
(beyond the permissions given by [API discovery roles](#discovery-roles)).
While far more secure, this can be disruptive to existing workloads expecting to automatically receive API permissions.
Here are two approaches for managing this transition: