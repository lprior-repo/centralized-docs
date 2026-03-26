---
doc_id: ref/docs-reference-kubectl-generated-kubectltaint.md/docs-reference-kubectl-generated-kubectltaint
chunk_id: ref/docs-reference-kubectl-generated-kubectltaint.md/docs-reference-kubectl-generated-kubectltaint#16-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 128
summary: [http://golang.org/pkg/text/template/#pkg-overview]. | |--validate string[=\"strict\"]Default: \"strict\"| || Must be one of: strict (or true), warn, ignore (or false). \"true\" or \"strict\" will use a...
---

[http://golang.org/pkg/text/template/#pkg-overview].
|
|--validate string[="strict"]Default: "strict"|
||
Must be one of: strict (or true), warn, ignore (or false). "true" or "strict" will use a schema to validate the input and fail the request if invalid. It will perform server side validation if ServerSideFieldValidation is enabled on the api-server, but will fall back to less reliable client-side validation if not. "warn" will warn about unknown or duplicate fields without blocking the request if server-side field validation is enabled on the API server, and behave as