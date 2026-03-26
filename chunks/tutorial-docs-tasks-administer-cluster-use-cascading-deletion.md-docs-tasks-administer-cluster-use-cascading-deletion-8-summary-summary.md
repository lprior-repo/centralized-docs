---
doc_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion
chunk_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion#8-summary
chunk_level: summary
chunk_type: prose
heading: Use foreground cascading deletion
token_count: 126
summary: ## Use foreground cascading deletion By default, Kubernetes uses [background cascading deletion](/docs/concepts/architecture/garbage-collection/#background-deletion) to delete dependents of an...
---

## Use foreground cascading deletion
By default, Kubernetes uses [background cascading deletion](/docs/concepts/architecture/garbage-collection/#background-deletion)
to delete dependents of an object. You can switch to foreground cascading deletion
using either `kubectl` or the Kubernetes API, depending on the Kubernetes
version your cluster runs.
To check the version, enter `kubectl version`.
You can delete objects using foreground cascading deletion using `kubectl` or the
Kubernetes API.
**Using kubectl**
Run the following command:
```
`kubectl delete deployment nginx-deployment --cascade=foreground
`
```