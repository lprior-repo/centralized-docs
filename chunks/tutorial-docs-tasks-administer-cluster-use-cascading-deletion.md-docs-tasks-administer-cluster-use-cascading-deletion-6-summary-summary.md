---
doc_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion
chunk_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion#6-summary
chunk_level: summary
chunk_type: prose
heading: Check owner references on your pods
token_count: 54
summary: ## Check owner references on your pods Check that the `ownerReferences` field is present on your pods: ``` `kubectl get pods -l app=nginx --output=yaml ` ``` The output has an `ownerReferences` field...
---

## Check owner references on your pods
Check that the `ownerReferences` field is present on your pods:
```
`kubectl get pods -l app=nginx --output=yaml
`
```
The output has an `ownerReferences` field similar to this: