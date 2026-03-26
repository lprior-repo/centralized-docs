---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#13-standard
chunk_level: standard
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 479
summary: * 'authorizer' - A CEL Authorizer. May be used to perform authorization checks for the principal (user or service account) of the request. See...
---

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