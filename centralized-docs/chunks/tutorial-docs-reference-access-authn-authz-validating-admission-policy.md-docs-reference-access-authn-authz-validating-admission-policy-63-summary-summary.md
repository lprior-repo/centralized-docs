---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#63-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 64
summary: * 'params' - Parameter resource referred to by the policy binding being evaluated. The value is null if `ParamKind` is not specified. * `namespaceObject` - The namespace, as a Kubernetes resource,...
---

* 'params' - Parameter resource referred to by the policy binding being evaluated. The value is
null if `ParamKind` is not specified.
* `namespaceObject` - The namespace, as a Kubernetes resource, that the incoming object belongs to.
The value is null if the incoming object is cluster-scoped.