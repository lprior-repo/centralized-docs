---
doc_id: ref/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutrestart.md/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutrestart
chunk_id: ref/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutrestart.md/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutrestart#1-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 80
summary: ## Synopsis Restart a resource. ``` ` Resource rollout will be restarted. ` ``` ``` `kubectl rollout restart RESOURCE ` ``` ## Examples ``` ` # Restart all deployments in the test-namespace namespace...
---

## Synopsis
Restart a resource.
```
` Resource rollout will be restarted.
`
```
```
`kubectl rollout restart RESOURCE
`
```
## Examples
```
` # Restart all deployments in the test-namespace namespace
kubectl rollout restart deployment -n test-namespace
# Restart deployments with the app=nginx label
kubectl rollout restart deployment --selector=app=nginx
`
```