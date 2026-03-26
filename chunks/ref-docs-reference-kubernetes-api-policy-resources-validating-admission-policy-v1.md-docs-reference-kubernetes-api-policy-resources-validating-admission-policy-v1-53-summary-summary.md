---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#53-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 110
summary: * 'authorizer.requestResource' - A CEL ResourceCheck constructed from the 'authorizer' and configured with the request resource. The `apiVersion`, `kind`, `metadata.name` and `metadata.generateName`...
---

* 'authorizer.requestResource' - A CEL ResourceCheck constructed from the 'authorizer' and configured with the
request resource.
The `apiVersion`, `kind`, `metadata.name` and `metadata.generateName` are always accessible from the root of the object. No other metadata properties are accessible.
Only property names of the form `[a-zA-Z\_.-/][a-zA-Z0-9\_.-/]\*` are accessible. Accessible property names are escaped according to the following rules when accessed in the expression: - '