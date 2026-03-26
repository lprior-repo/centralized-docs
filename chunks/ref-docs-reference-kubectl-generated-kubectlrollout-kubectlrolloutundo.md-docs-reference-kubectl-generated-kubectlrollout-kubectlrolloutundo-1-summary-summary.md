---
doc_id: ref/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutundo.md/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutundo
chunk_id: ref/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutundo.md/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutundo#1-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 100
summary: ## Synopsis Roll back to a previous rollout. ``` `kubectl rollout undo (TYPE NAME | TYPE/NAME) [flags] ` ``` ## Examples ``` ` # Roll back to the previous deployment kubectl rollout undo...
---

## Synopsis
Roll back to a previous rollout.
```
`kubectl rollout undo (TYPE NAME | TYPE/NAME) [flags]
`
```
## Examples
```
` # Roll back to the previous deployment
kubectl rollout undo deployment/abc
# Roll back to daemonset revision 3
kubectl rollout undo daemonset/abc --to-revision=3
# Roll back to the previous deployment with dry-run
kubectl rollout undo --dry-run=server deployment/abc
`
```