---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#12-standard
chunk_level: standard
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 293
summary: * **spec.validations.expression** (string), required Expression represents the expression which will be evaluated by CEL. ref: [https://github.com/google/cel-spec](https://github.com/google/cel-spec)...
---

* **spec.validations.expression** (string), required
Expression represents the expression which will be evaluated by CEL. ref: [https://github.com/google/cel-spec](https://github.com/google/cel-spec) CEL expressions have access to the contents of the API request/response, organized into CEL variables as well as some other useful variables:
* 'object' - The object from the incoming request. The value is null for DELETE requests. - 'oldObject' - The existing object. The value is null for CREATE requests. - 'request' - Attributes of the API request([ref](/pkg/apis/admission/types.go#AdmissionRequest)). - 'params' - Parameter resource referred to by the policy binding being evaluated. Only populated if the policy has a ParamKind. - 'namespaceObject' - The namespace object that the incoming object belongs to. The value is null for cluster-scoped resources. - 'variables' - Map of composited variables, from its name to its lazily evaluated value.
For example, a variable named 'foo' can be accessed as 'variables.foo'.
* 'authorizer' - A CEL Authorizer. May be used to perform authorization checks for the principal (user or service account) of the request.
See [https://pkg.go.dev/k8s.io/apiserver/pkg/cel/library#Authz](https://pkg.go.dev/k8s.io/apiserver/pkg/cel/library#Authz)