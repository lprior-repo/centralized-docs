---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#6-detailed
chunk_level: detailed
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 995
summary: * **spec.paramKind.apiVersion** (string) APIVersion is the API group version the resources belong to. In format of \"group/version\". Required. * **spec.paramKind.kind** (string) Kind is the API kind...
---

* **spec.paramKind.apiVersion** (string)
APIVersion is the API group version the resources belong to. In format of "group/version". Required.
* **spec.paramKind.kind** (string)
Kind is the API kind the resources belong to. Required.
* **spec.validations** ([]Validation)
*Atomic: will be replaced during a merge*
Validations contain CEL expressions which is used to apply the validation. Validations and AuditAnnotations may not both be empty; a minimum of one Validations or AuditAnnotations is required.
*Validation specifies the CEL expression which is used to apply the validation.*
* **spec.validations.expression** (string), required
Expression represents the expression which will be evaluated by CEL. ref: [https://github.com/google/cel-spec](https://github.com/google/cel-spec) CEL expressions have access to the contents of the API request/response, organized into CEL variables as well as some other useful variables:
* 'object' - The object from the incoming request. The value is null for DELETE requests. - 'oldObject' - The existing object. The value is null for CREATE requests. - 'request' - Attributes of the API request([ref](/pkg/apis/admission/types.go#AdmissionRequest)). - 'params' - Parameter resource referred to by the policy binding being evaluated. Only populated if the policy has a ParamKind. - 'namespaceObject' - The namespace object that the incoming object belongs to. The value is null for cluster-scoped resources. - 'variables' - Map of composited variables, from its name to its lazily evaluated value.
For example, a variable named 'foo' can be accessed as 'variables.foo'.
* 'authorizer' - A CEL Authorizer. May be used to perform authorization checks for the principal (user or service account) of the request.
See [https://pkg.go.dev/k8s.io/apiserver/pkg/cel/library#Authz](https://pkg.go.dev/k8s.io/apiserver/pkg/cel/library#Authz)
* 'authorizer.requestResource' - A CEL ResourceCheck constructed from the 'authorizer' and configured with the
request resource.
The `apiVersion`, `kind`, `metadata.name` and `metadata.generateName` are always accessible from the root of the object. No other metadata properties are accessible.
Only property names of the form `[a-zA-Z\_.-/][a-zA-Z0-9\_.-/]\*` are accessible. Accessible property names are escaped according to the following rules when accessed in the expression: - '**' escapes to '**underscores**' - '.' escapes to '**dot**' - '-' escapes to '**dash**' - '/' escapes to '**slash**' - Property names that exactly match a CEL RESERVED keyword escape to '**{keyword}\_\_'. The keywords are:
"true", "false", "null", "in", "as", "break", "const", "continue", "else", "for", "function", "if",
"import", "let", "loop", "package", "namespace", "return".
Examples:
* Expression accessing a property named "namespace": {"Expression": "object.**namespace** &gt; 0"}
* Expression accessing a property named "x-prop": {"Expression": "object.x\_\_dash\_\_prop &gt; 0"}
* Expression accessing a property named "redact\_\_d": {"Expression": "object.redact\_\_underscores\_\_d &gt; 0"}
Equality on arrays with list type of 'set' or 'map' ignores element order, i.e. [1, 2] == [2, 1]. Concatenation on arrays with x-kubernetes-list-type use the semantics of the list type:
* 'set': `X + Y` performs a union where the array positions of all elements in `X` are preserved and
non-intersecting elements in `Y` are appended, retaining their partial order.
* 'map': `X + Y` performs a merge where the array positions of all keys in `X` are preserved but the values
are overwritten by values in `Y` when the key sets of `X` and `Y` intersect. Elements in `Y` with
non-intersecting keys are appended, retaining their partial order.
Required.
* **spec.validations.message** (string)
Message represents the message displayed when validation fails. The message is required if the Expression contains line breaks. The message must not contain line breaks. If unset, the message is "failed rule: {Rule}". e.g. "must be a URL with the host matching spec.host" If the Expression contains line breaks. Message is required. The message must not contain line breaks. If unset, the message is "failed Expression: {Expression}".