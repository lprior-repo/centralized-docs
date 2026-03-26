---
doc_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateserviceloadbalancer.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateserviceloadbalancer
chunk_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateserviceloadbalancer.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateserviceloadbalancer#1-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 38
summary: ## Examples ``` ` # Create a new LoadBalancer service named my-lbs kubectl create service loadbalancer my-lbs --tcp=5678:8080 ` ```
---

## Examples
```
` # Create a new LoadBalancer service named my-lbs
kubectl create service loadbalancer my-lbs --tcp=5678:8080
`
```