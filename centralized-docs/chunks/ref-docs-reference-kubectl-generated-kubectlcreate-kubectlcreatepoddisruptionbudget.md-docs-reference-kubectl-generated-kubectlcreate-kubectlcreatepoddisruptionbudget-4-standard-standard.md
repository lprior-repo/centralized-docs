---
doc_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget
chunk_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget#4-standard
chunk_level: standard
chunk_type: prose
heading: Options
token_count: 110
summary: \"true\" or \"strict\" will use a schema to validate the input and fail the request if invalid. It will perform server side validation if ServerSideFieldValidation is enabled on the api-server, but will...
---

"true" or "strict" will use a schema to validate the input and fail the request if invalid. It will perform server side validation if ServerSideFieldValidation is enabled on the api-server, but will fall back to less reliable client-side validation if not. "warn" will warn about unknown or duplicate fields without blocking the request if server-side field validation is enabled on the API server, and behave as "ignore" otherwise. "false" or "ignore" will not perform any schema validation, silently dropping any unknown or duplicate fields.
|