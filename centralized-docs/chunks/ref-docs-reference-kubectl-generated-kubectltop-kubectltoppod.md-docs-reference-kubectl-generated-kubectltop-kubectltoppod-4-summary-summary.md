---
doc_id: ref/docs-reference-kubectl-generated-kubectltop-kubectltoppod.md/docs-reference-kubectl-generated-kubectltop-kubectltoppod
chunk_id: ref/docs-reference-kubectl-generated-kubectltop-kubectltoppod.md/docs-reference-kubectl-generated-kubectltop-kubectltoppod#4-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 84
summary: ## Examples ``` ` # Show metrics for all pods in the default namespace kubectl top pod # Show metrics for all pods in the given namespace kubectl top pod --namespace=NAMESPACE # Show metrics for a...
---

## Examples
```
` # Show metrics for all pods in the default namespace
kubectl top pod
# Show metrics for all pods in the given namespace
kubectl top pod --namespace=NAMESPACE
# Show metrics for a given pod and its containers
kubectl top pod POD\_NAME --containers
# Show metrics for the pods defined by label name=myLabel
kubectl top pod -l name=myLabel
`
```