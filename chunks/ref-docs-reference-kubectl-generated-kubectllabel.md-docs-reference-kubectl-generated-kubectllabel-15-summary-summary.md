---
doc_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel
chunk_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel#15-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 121
summary: --selector string| || Selector (label query) to filter on, supports '=', '==', '!=', 'in', 'notin'.(e.g. -l key1=value1,key2=value2,key3 in (value3)). Matching objects must satisfy all of the...
---

--selector string|
||
Selector (label query) to filter on, supports '=', '==', '!=', 'in', 'notin'.(e.g. -l key1=value1,key2=value2,key3 in (value3)). Matching objects must satisfy all of the specified label constraints.
|
|--show-managed-fields|
||
If true, keep the managedFields when printing objects in JSON or YAML format.
|
|--template string|
||
Template string or path to template file to use when -o=go-template, -o=go-template-file. The template format is golang templates [