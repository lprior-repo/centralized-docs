---
doc_id: ref/docs-reference-kubectl-generated-kubectldescribe.md/docs-reference-kubectl-generated-kubectldescribe
chunk_id: ref/docs-reference-kubectl-generated-kubectldescribe.md/docs-reference-kubectl-generated-kubectldescribe#9-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 106
summary: --help| || help for describe | |-k, --kustomize string| || Process the kustomization directory. This flag can't be used together with -f or -R. | |-R, --recursive| || Process the directory used in...
---

--help|
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
Selector (label query) to filter on, supports '=', '==', '!=', 'in', 'notin'