---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#49-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 47
summary: 1. Making `roleRef` immutable allows granting someone `update` permission on an existing binding object, so that they can manage the list of subjects, without being able to change the role that is...
---

1. Making `roleRef` immutable allows granting someone `update` permission on an existing binding
object, so that they can manage the list of subjects, without being able to change
the role that is granted to those subjects.