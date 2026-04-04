---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#58-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 126
summary: You cannot restrict **deletecollection** or top-level **create** requests by resource name. For **create**, this limitation is because the name of the new object may not be known at authorization...
---

You cannot restrict **deletecollection** or top-level **create** requests by resource name.
For **create**, this limitation is because the name of the new object may not be known at authorization time. However, the **create** limitation applies only to top-level resources, not subresources. For example, you can use the `resourceNames` field with `pods/exec`.
If you restrict **list** or **watch** by `resourceName`, clients must include a `metadata.name` field selector in their **list** or **watch** request (that matches the specified `resourceName`)
in order to be authorized.