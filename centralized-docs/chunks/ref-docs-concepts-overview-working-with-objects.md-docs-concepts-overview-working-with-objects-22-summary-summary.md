---
doc_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects
chunk_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects#22-summary
chunk_level: summary
chunk_type: prose
heading: Server side field validation
token_count: 114
summary: `true` (equivalent to `strict`) and `false` (equivalent to `ignore`). The default validation setting for `kubectl` is `--validate=true`. `Strict`Strict field validation, errors on validation...
---

`true` (equivalent to `strict`)
and `false` (equivalent to `ignore`). The default validation setting for `kubectl` is `--validate=true`.
`Strict`Strict field validation, errors on validation failure`Warn`Field validation is performed, but errors are exposed as warnings rather than failing the request`Ignore`No server side field validation is performed
When `kubectl` cannot connect to an API server that supports field validation it will fall back
to using client-side validation. Kubernetes 1.27 and later versions always offer field validation;