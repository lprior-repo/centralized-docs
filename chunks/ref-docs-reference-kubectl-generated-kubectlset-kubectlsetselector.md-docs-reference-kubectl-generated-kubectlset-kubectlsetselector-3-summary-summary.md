---
doc_id: ref/docs-reference-kubectl-generated-kubectlset-kubectlsetselector.md/docs-reference-kubectl-generated-kubectlset-kubectlsetselector
chunk_id: ref/docs-reference-kubectl-generated-kubectlset-kubectlsetselector.md/docs-reference-kubectl-generated-kubectlset-kubectlsetselector#3-summary
chunk_level: summary
chunk_type: table
heading: Examples
token_count: 108
summary: ## Examples ``` ` # Set the labels and selector before creating a deployment/service pair kubectl create service clusterip my-svc --clusterip=\"None\" -o yaml --dry-run=client | kubectl set selector...
---

## Examples
```
` # Set the labels and selector before creating a deployment/service pair
kubectl create service clusterip my-svc --clusterip="None" -o yaml --dry-run=client | kubectl set selector --local -f - 'environment=qa' -o yaml | kubectl create -f -
kubectl create deployment my-dep --image=nginx -o yaml --dry-run=client | kubectl label --local -f - environment=qa -o yaml | kubectl create -f -
`
```