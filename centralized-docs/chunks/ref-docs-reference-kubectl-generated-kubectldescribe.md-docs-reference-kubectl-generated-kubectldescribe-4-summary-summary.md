---
doc_id: ref/docs-reference-kubectl-generated-kubectldescribe.md/docs-reference-kubectl-generated-kubectldescribe
chunk_id: ref/docs-reference-kubectl-generated-kubectldescribe.md/docs-reference-kubectl-generated-kubectldescribe#4-summary
chunk_level: summary
chunk_type: prose
heading: Synopsis
token_count: 125
summary: ## Synopsis Show details of a specific resource or group of resources. Print a detailed description of the selected resources, including related resources such as events or controllers. You may...
---

## Synopsis
Show details of a specific resource or group of resources.
Print a detailed description of the selected resources, including related resources such as events or controllers. You may select a single object by name, all objects of that type, provide a name prefix, or label selector. For example:
```
` $ kubectl describe TYPE NAME\_PREFIX
`
```
will first check for an exact match on TYPE and NAME\_PREFIX. If no such resource exists, it will output details for every resource that has a name prefixed with NAME\_PREFIX.
Use "kubectl api-resources" for a complete list of supported resources.