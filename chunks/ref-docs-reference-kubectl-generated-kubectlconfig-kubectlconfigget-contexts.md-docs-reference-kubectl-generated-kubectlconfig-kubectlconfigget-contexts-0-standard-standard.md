---
doc_id: ref/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigget-contexts.md/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigget-contexts
chunk_id: ref/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigget-contexts.md/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigget-contexts#0-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 214
summary: ## Table of Contents    - [Synopsis](#synopsis)   - [Examples](#examples) - [Describe one context in your kubeconfig file](#describe-one-context-in-your-kubeconfig-file)   - [Options](#options)   -...
---

## Table of Contents

  - [Synopsis](#synopsis)
  - [Examples](#examples)
- [Describe one context in your kubeconfig file](#describe-one-context-in-your-kubeconfig-file)
  - [Options](#options)
  - [Parent Options Inherited](#parent-options-inherited)
  - [Feedback](#feedback)

---

## Synopsis
Display one or many contexts from the kubeconfig file.
```
`kubectl config get-contexts [(-o|--output=)name)]
`
```
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