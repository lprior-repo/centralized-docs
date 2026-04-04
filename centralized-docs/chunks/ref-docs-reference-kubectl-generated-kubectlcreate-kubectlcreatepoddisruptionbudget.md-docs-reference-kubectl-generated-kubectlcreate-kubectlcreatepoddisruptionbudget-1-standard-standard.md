---
doc_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget
chunk_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget#1-standard
chunk_level: standard
chunk_type: table
heading: Examples
token_count: 198
summary: # kubectl create poddisruptionbudget Create a pod disruption budget with the specified name ## Synopsis Create a pod disruption budget with the specified name, selector, and desired minimum available...
---

# kubectl create poddisruptionbudget
Create a pod disruption budget with the specified name
## Synopsis
Create a pod disruption budget with the specified name, selector, and desired minimum available pods.
```
`kubectl create poddisruptionbudget NAME --selector=SELECTOR --min-available=N [--dry-run=server|client|none]
`
```
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