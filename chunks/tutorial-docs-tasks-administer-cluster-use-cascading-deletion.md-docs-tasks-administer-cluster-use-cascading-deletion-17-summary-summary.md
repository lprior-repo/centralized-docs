---
doc_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion
chunk_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion#17-summary
chunk_level: summary
chunk_type: prose
heading: Delete owner objects and orphan dependents
token_count: 123
summary: ## Delete owner objects and orphan dependents By default, when you tell Kubernetes to delete an object, the [controller](/docs/concepts/architecture/controller/) also deletes dependent objects. You...
---

## Delete owner objects and orphan dependents
By default, when you tell Kubernetes to delete an object, the
[controller](/docs/concepts/architecture/controller/) also deletes
dependent objects. You can make Kubernetes *orphan* these dependents using
`kubectl` or the Kubernetes API, depending on the Kubernetes version your
cluster runs.
To check the version, enter `kubectl version`.
**Using kubectl**
Run the following command:
```
`kubectl delete deployment nginx-deployment --cascade=orphan
`
```
**Using the Kubernetes API**
1. Start a local proxy session: