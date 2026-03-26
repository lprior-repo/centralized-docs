---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#62-summary
chunk_level: summary
chunk_type: prose
heading: Auditing
token_count: 68
summary: ## Auditing An audit event is logged for each impersonation request to help track how impersonation is used. When a request uses constrained impersonation, the audit event includes...
---

## Auditing
An audit event is logged for each impersonation request to help track how impersonation is used.
When a request uses constrained impersonation, the audit event includes `authenticationMetadata`
object with an `impersonationConstraint` field that indicates which constrained impersonation verb
was used to authorize the request.
Example audit event: