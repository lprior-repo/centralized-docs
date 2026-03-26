---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#54-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 41
summary: #### Example: Node agent impersonating the associated node This is a common pattern for node agents (like CNI plugins) that need to read pods on their node without having cluster-wide pod access.
---

#### Example: Node agent impersonating the associated node
This is a common pattern for node agents (like CNI plugins) that need to read pods on their node
without having cluster-wide pod access.