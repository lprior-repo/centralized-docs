---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#157-summary
chunk_level: summary
chunk_type: prose
heading: Upgrading from ABAC
token_count: 106
summary: (`--vmodule=rbac\*=5` or `--v=5`), you can see RBAC denials in the API server log (prefixed with `RBAC`). You can use that information to determine which roles need to be granted to which users,...
---

(`--vmodule=rbac\*=5` or `--v=5`), you can see RBAC denials in the API server log
(prefixed with `RBAC`).
You can use that information to determine which roles need to be granted to which users, groups, or service accounts.
Once you have [granted roles to service accounts](#service-account-permissions) and workloads
are running with no RBAC denial messages in the server logs, you can remove the ABAC authorizer.