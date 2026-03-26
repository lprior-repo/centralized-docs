---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#37-standard
chunk_level: standard
chunk_type: prose
heading: Upgrading from ABAC
token_count: 391
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
### Parallel authorizers
Run both the RBAC and ABAC authorizers, and specify a policy file that contains
the [legacy ABAC policy](/docs/reference/access-authn-authz/abac/#policy-file-format):
```
`--authorization-mode=...,RBAC,ABAC --authorization-policy-file=mypolicy.json
`
```
To explain that first command line option in detail: if earlier authorizers, such as Node,
deny a request, then the RBAC authorizer attempts to authorize the API request. If RBAC
also denies that API request, the ABAC authorizer is then run. This means that any request
allowed by *either* the RBAC or ABAC policies is allowed.
When the kube-apiserver is run with a log level of 5 or higher for the RBAC component
(`--vmodule=rbac\*=5` or `--v=5`), you can see RBAC denials in the API server log
(prefixed with `RBAC`).
You can use that information to determine which roles need to be granted to which users, groups, or service accounts.
Once you have [granted roles to service accounts](#service-account-permissions) and workloads
are running with no RBAC denial messages in the server logs, you can remove the ABAC authorizer.