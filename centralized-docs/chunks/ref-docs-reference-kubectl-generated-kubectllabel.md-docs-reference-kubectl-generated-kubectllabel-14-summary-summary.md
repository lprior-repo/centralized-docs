---
doc_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel
chunk_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel#14-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 125
summary: | |--overwrite| || If true, allow labels to be overwritten, otherwise reject label updates that overwrite existing labels. | |-R, --recursive| || Process the directory used in -f, --filename...
---

|
|--overwrite|
||
If true, allow labels to be overwritten, otherwise reject label updates that overwrite existing labels.
|
|-R, --recursive|
||
Process the directory used in -f, --filename recursively. Useful when you want to manage related manifests organized within the same directory.
|
|--resource-version string|
||
If non-empty, the labels update will only succeed if this is the current resource-version for the object. Only valid when specifying a single resource.
|
|-l, --selector string|
||
Selector (label query) to filter on, supports '=', '==', '!=', 'in', 'notin'