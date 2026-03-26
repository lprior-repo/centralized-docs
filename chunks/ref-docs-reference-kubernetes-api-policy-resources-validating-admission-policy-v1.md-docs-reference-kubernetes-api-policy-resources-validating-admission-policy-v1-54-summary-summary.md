---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#54-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 128
summary: are accessible. Accessible property names are escaped according to the following rules when accessed in the expression: - '**' escapes to '**underscores**' - '.' escapes to '**dot**' - '-' escapes to...
---

 are accessible. Accessible property names are escaped according to the following rules when accessed in the expression: - '**' escapes to '**underscores**' - '.' escapes to '**dot**' - '-' escapes to '**dash**' - '/' escapes to '**slash**' - Property names that exactly match a CEL RESERVED keyword escape to '**{keyword}\_\_'. The keywords are:
"true", "false", "null", "in", "as", "break", "const", "continue", "else", "for", "function", "if",
"import", "let", "loop", "package", "namespace",