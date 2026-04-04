---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#13-standard
chunk_level: standard
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 444
summary: * 'object' - The object from the incoming request. The value is null for DELETE requests. * 'oldObject' - The existing object. The value is null for CREATE requests. * 'request' - Attributes of the...
---

* 'object' - The object from the incoming request. The value is null for DELETE requests.
* 'oldObject' - The existing object. The value is null for CREATE requests.
* 'request' - Attributes of the [admission request](/docs/reference/config-api/apiserver-admission.v1/#admission-k8s-io-v1-AdmissionRequest).
* 'params' - Parameter resource referred to by the policy binding being evaluated. The value is
null if `ParamKind` is not specified.
* `namespaceObject` - The namespace, as a Kubernetes resource, that the incoming object belongs to.
The value is null if the incoming object is cluster-scoped.
* `authorizer` - A CEL Authorizer. May be used to perform authorization checks for the principal
(authenticated user) of the request. See
[AuthzSelectors](https://pkg.go.dev/k8s.io/apiserver/pkg/cel/library#AuthzSelectors) and
[Authz](https://pkg.go.dev/k8s.io/apiserver/pkg/cel/library#Authz) in the Kubernetes CEL library
documentation for more details.
* `authorizer.requestResource` - A shortcut for an authorization check configured with the request
resource (group, resource, (subresource), namespace, name).
In CEL expressions, variables like `object` and `oldObject` are strongly-typed.
You can access any field in the object's schema, such as `object.metadata.labels` and fields in `spec`.
For any Kubernetes object, including schemaless Custom Resources, CEL guarantees access to a minimal set of properties:
`apiVersion`, `kind`, `metadata.name`, and `metadata.generateName`.
Equality on arrays with list type of 'set' or 'map' ignores element order, i.e. [1, 2] == [2, 1].
Concatenation on arrays with x-kubernetes-list-type use the semantics of the list type:
* 'set': `X + Y` performs a union where the array positions of all elements in `X` are preserved and
non-intersecting elements in `Y` are appended, retaining their partial order.