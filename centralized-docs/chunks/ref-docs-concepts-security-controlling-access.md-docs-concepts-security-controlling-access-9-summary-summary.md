---
doc_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access
chunk_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access#9-summary
chunk_level: summary
chunk_type: prose
heading: Authorization
token_count: 104
summary: ## Authorization After the request is authenticated as coming from a specific user, the request must be authorized. This is shown as step**2**in the diagram. A request must include the username of...
---

## Authorization
After the request is authenticated as coming from a specific user, the request must
be authorized. This is shown as step**2**in the diagram.
A request must include the username of the requester, the requested action, and
the object affected by the action. The request is authorized if an existing policy
declares that the user has permissions to complete the requested action.
For example, if Bob has the policy below, then he can read pods only in the namespace `projectCaribou`: