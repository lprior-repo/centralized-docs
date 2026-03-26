---
doc_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateserviceloadbalancer.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateserviceloadbalancer
chunk_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateserviceloadbalancer.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateserviceloadbalancer#0-standard
chunk_level: standard
chunk_type: table
heading: Examples
token_count: 135
summary: ## Table of Contents    - [Synopsis](#synopsis)   - [Examples](#examples)   - [Options](#options)   - [Parent Options Inherited](#parent-options-inherited)   - [Feedback](#feedback)  ---  ## Synopsis...
---

## Table of Contents

  - [Synopsis](#synopsis)
  - [Examples](#examples)
  - [Options](#options)
  - [Parent Options Inherited](#parent-options-inherited)
  - [Feedback](#feedback)

---

## Synopsis
Create a LoadBalancer service with the specified name.
```
`kubectl create service loadbalancer NAME [--tcp=port:targetPort] [--dry-run=server|client|none]
`
```
## Examples
```
` # Create a new LoadBalancer service named my-lbs
kubectl create service loadbalancer my-lbs --tcp=5678:8080
`
```