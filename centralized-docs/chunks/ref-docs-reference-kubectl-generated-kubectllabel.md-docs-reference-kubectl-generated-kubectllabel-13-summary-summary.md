---
doc_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel
chunk_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel#13-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 128
summary: || Process the kustomization directory. This flag can't be used together with -f or -R. | |--list| || If true, display the labels for a given resource. | |--local| || If true, label will NOT contact...
---

||
Process the kustomization directory. This flag can't be used together with -f or -R.
|
|--list|
||
If true, display the labels for a given resource.
|
|--local|
||
If true, label will NOT contact api-server but run locally.
|
|-o, --output string|
||
Output format. One of: (json, yaml, kyaml, name, go-template, go-template-file, template, templatefile, jsonpath, jsonpath-as-json, jsonpath-file).
|
|--overwrite|
||
If true, allow labels to be overwritten, otherwise reject label updates that overwrite existing labels.