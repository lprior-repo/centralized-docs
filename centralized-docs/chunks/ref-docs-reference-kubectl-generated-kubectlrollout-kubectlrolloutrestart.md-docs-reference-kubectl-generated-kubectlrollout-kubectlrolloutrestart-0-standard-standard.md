---
doc_id: ref/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutrestart.md/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutrestart
chunk_id: ref/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutrestart.md/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutrestart#0-standard
chunk_level: standard
chunk_type: prose
heading: Examples
token_count: 155
summary: ## Table of Contents    - [Synopsis](#synopsis)   - [Examples](#examples) - [Restart deployments with the app=nginx label](#restart-deployments-with-the-appnginx-label)   - [Options](#options)   -...
---

## Table of Contents

  - [Synopsis](#synopsis)
  - [Examples](#examples)
- [Restart deployments with the app=nginx label](#restart-deployments-with-the-appnginx-label)
  - [Options](#options)
  - [Parent Options Inherited](#parent-options-inherited)
  - [Feedback](#feedback)

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