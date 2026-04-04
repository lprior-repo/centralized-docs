---
doc_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget
chunk_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget#4-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 126
summary: ## Examples ``` ` # Create a pod disruption budget named my-pdb that will select all pods with the app=rails label # and require at least one of them being available at any point in time kubectl...
---

## Examples
```
` # Create a pod disruption budget named my-pdb that will select all pods with the app=rails label
# and require at least one of them being available at any point in time
kubectl create poddisruptionbudget my-pdb --selector=app=rails --min-available=1
# Create a pod disruption budget named my-pdb that will select all pods with the app=nginx label
# and require at least half of the pods selected to be available at any point in time
kubectl create pdb my-pdb --selector=app=nginx --min-available=50%
`
```