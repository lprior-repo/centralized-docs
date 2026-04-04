---
doc_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget
chunk_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget#3-summary
chunk_level: summary
chunk_type: table
heading: Synopsis
token_count: 71
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