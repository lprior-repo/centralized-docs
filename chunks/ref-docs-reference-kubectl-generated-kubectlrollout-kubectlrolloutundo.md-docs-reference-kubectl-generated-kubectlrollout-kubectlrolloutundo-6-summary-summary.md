---
doc_id: ref/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutundo.md/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutundo
chunk_id: ref/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutundo.md/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutundo#6-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 97
summary: | |--show-managed-fields| || If true, keep the managedFields when printing objects in JSON or YAML format. | |--template string| || Template string or path to template file to use when...
---

|
|--show-managed-fields|
||
If true, keep the managedFields when printing objects in JSON or YAML format.
|
|--template string|
||
Template string or path to template file to use when -o=go-template, -o=go-template-file. The template format is golang templates [http://golang.org/pkg/text/template/#pkg-overview].
|
|--to-revision int|
||
The revision to rollback to. Default to 0 (last revision).
|