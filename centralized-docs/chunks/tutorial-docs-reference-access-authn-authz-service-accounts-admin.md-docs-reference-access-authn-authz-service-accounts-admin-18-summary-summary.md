---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#18-summary
chunk_level: summary
chunk_type: prose
heading: User accounts versus service accounts
token_count: 125
summary: ### Additional metadata in Pod bound tokens FEATURE STATE: `Kubernetes v1.32 [stable]`(enabled by default) When a service account token is bound to a Pod object, additional metadata is also embedded...
---

### Additional metadata in Pod bound tokens
FEATURE STATE:
`Kubernetes v1.32 [stable]`(enabled by default)
When a service account token is bound to a Pod object, additional metadata is also
embedded into the token that indicates the value of the bound pod's `spec.nodeName` field,
and the uid of that Node, if available.
This node information is **not** verified by the kube-apiserver when the token is used for authentication.
It is included so integrators do not have to fetch Pod or Node API objects to check the associated Node name
and uid when inspecting a JWT.