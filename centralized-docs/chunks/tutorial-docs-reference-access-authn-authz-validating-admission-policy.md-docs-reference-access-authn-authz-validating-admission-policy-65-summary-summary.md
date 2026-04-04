---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#65-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 128
summary: * `authorizer.requestResource` - A shortcut for an authorization check configured with the request resource (group, resource, (subresource), namespace, name). In CEL expressions, variables like...
---

* `authorizer.requestResource` - A shortcut for an authorization check configured with the request
resource (group, resource, (subresource), namespace, name).
In CEL expressions, variables like `object` and `oldObject` are strongly-typed.
You can access any field in the object's schema, such as `object.metadata.labels` and fields in `spec`.
For any Kubernetes object, including schemaless Custom Resources, CEL guarantees access to a minimal set of properties:
`apiVersion`, `kind`, `metadata.name`, and `metadata.generateName`.
Equality on arrays with list type of 'set' or 'map'