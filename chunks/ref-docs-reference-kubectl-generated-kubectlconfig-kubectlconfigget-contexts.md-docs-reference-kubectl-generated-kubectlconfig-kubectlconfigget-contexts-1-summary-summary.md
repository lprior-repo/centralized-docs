---
doc_id: ref/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigget-contexts.md/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigget-contexts
chunk_id: ref/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigget-contexts.md/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigget-contexts#1-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 105
summary: ## Examples ``` ` # List all the contexts in your kubeconfig file kubectl config get-contexts # Describe one context in your kubeconfig file kubectl config get-contexts my-context ` ``` ## Options...
---

## Examples
```
` # List all the contexts in your kubeconfig file
kubectl config get-contexts
# Describe one context in your kubeconfig file
kubectl config get-contexts my-context
`
```
## Options
|-h, --help|
||
help for get-contexts
|
|--no-headers|
||
When using the default or custom-column output format, don't print headers (default print headers).
|
|-o, --output string|
||
Output format. One of: (name).
|