---
doc_id: ref/docs-reference-kubectl-conventions.md/docs-reference-kubectl-conventions
chunk_id: ref/docs-reference-kubectl-conventions.md/docs-reference-kubectl-conventions#3-summary
chunk_level: summary
chunk_type: prose
heading: Using `kubectl` in Reusable Scripts
token_count: 91
summary: * Don't rely on context, preferences, or other implicit states.## Subresources * You can use the `--subresource` argument for kubectl subcommands such as `get`, `patch`, `edit`, `apply` and `replace`...
---

* Don't rely on context, preferences, or other implicit states.## Subresources
* You can use the `--subresource` argument for kubectl subcommands such as `get`, `patch`,
`edit`, `apply` and `replace` to fetch and update subresources for all resources that
support them. In Kubernetes version 1.35, only the `status`, `scale`
and `resize` subresources are supported.