---
doc_id: ref/docs-reference-kubectl-generated-kubectldescribe.md/docs-reference-kubectl-generated-kubectldescribe
chunk_id: ref/docs-reference-kubectl-generated-kubectldescribe.md/docs-reference-kubectl-generated-kubectldescribe#2-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 252
summary: ## Options |-A, --all-namespaces| || If present, list the requested object(s) across all namespaces. Namespace in current context is ignored even if specified with --namespace. | |--chunk-size...
---

## Options
|-A, --all-namespaces|
||
If present, list the requested object(s) across all namespaces. Namespace in current context is ignored even if specified with --namespace.
|
|--chunk-size intDefault: 500|
||
Return large lists in chunks rather than all at once. Pass 0 to disable.
|
|-f, --filename strings|
||
Filename, directory, or URL to files containing the resource to describe
|
|-h, --help|
||
help for describe
|
|-k, --kustomize string|
||
Process the kustomization directory. This flag can't be used together with -f or -R.
|
|-R, --recursive|
||
Process the directory used in -f, --filename recursively. Useful when you want to manage related manifests organized within the same directory.
|
|-l, --selector string|
||
Selector (label query) to filter on, supports '=', '==', '!=', 'in', 'notin'.(e.g. -l key1=value1,key2=value2,key3 in (value3)). Matching objects must satisfy all of the specified label constraints.
|
|--show-eventsDefault: true|
||
If true, display events related to the described object.
|