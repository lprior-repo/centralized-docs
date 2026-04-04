---
doc_id: ref/docs-reference-kubectl-generated-kubectldescribe.md/docs-reference-kubectl-generated-kubectldescribe
chunk_id: ref/docs-reference-kubectl-generated-kubectldescribe.md/docs-reference-kubectl-generated-kubectldescribe#6-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 103
summary: ## Examples ``` ` # Describe a node kubectl describe nodes kubernetes-node-emt8.c.myproject.internal # Describe a pod identified by type and name in \"pod.json\" kubectl describe -f pod.json # Describe...
---

## Examples
```
` # Describe a node
kubectl describe nodes kubernetes-node-emt8.c.myproject.internal
# Describe a pod identified by type and name in "pod.json"
kubectl describe -f pod.json
# Describe pods by label name=myLabel
kubectl describe pods -l name=myLabel
# Describe all pods managed by the 'frontend' replication controller
# (rc-created pods get the name of the rc as a prefix in the pod name)
kubectl describe pods frontend
`
```