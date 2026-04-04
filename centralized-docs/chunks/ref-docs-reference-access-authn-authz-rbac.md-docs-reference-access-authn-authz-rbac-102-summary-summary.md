---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#102-summary
chunk_level: summary
chunk_type: table
heading: Default roles and role bindings
token_count: 121
summary: . You should use the [Node authorizer](/docs/reference/access-authn-authz/node/) and [NodeRestriction admission plugin](/docs/reference/access-authn-authz/admission-controllers/#noderestriction)...
---

.
You should use the [Node authorizer](/docs/reference/access-authn-authz/node/) and [NodeRestriction admission plugin](/docs/reference/access-authn-authz/admission-controllers/#noderestriction) instead of the system:node role, and allow granting API access to kubelets based on the Pods scheduled to run on them.
The system:node role only exists for compatibility with Kubernetes clusters upgraded from versions prior to v1.8.
|
|**system:node-proxier**|**system:kube-proxy** user|Allows access to the resources required by the