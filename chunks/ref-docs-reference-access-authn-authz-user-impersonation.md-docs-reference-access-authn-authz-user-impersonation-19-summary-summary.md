---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#19-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 72
summary: For impersonation, extra fields and impersonated UIDs are both under the \"authentication.k8s.io\" `apiGroup`. Extra fields are evaluated as sub-resources of the resource \"userextras\". To allow a user...
---

For impersonation, extra fields and impersonated UIDs are both under the "authentication.k8s.io" `apiGroup`.
Extra fields are evaluated as sub-resources of the resource "userextras". To
allow a user to use impersonation headers for the extra field `scopes` and
for UIDs, a user should be granted the following role: