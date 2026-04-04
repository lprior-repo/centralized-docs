---
doc_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects
chunk_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects#6-standard
chunk_level: standard
chunk_type: prose
heading: Server side field validation
token_count: 245
summary: ## Server side field validation Starting with Kubernetes v1.25, the API server offers server side [field validation](/docs/reference/using-api/api-concepts/#field-validation) that detects...
---

## Server side field validation
Starting with Kubernetes v1.25, the API server offers server side
[field validation](/docs/reference/using-api/api-concepts/#field-validation)
that detects unrecognized or duplicate fields in an object. It provides all the functionality
of `kubectl --validate` on the server side.
The `kubectl` tool uses the `--validate` flag to set the level of field validation. It accepts the
values `ignore`, `warn`, and `strict` while also accepting the values `true` (equivalent to `strict`)
and `false` (equivalent to `ignore`). The default validation setting for `kubectl` is `--validate=true`.
`Strict`Strict field validation, errors on validation failure`Warn`Field validation is performed, but errors are exposed as warnings rather than failing the request`Ignore`No server side field validation is performed
When `kubectl` cannot connect to an API server that supports field validation it will fall back
to using client-side validation. Kubernetes 1.27 and later versions always offer field validation;
older Kubernetes releases might not. If your cluster is older than v1.27, check the documentation
for your version of Kubernetes.