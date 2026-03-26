---
doc_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion
chunk_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion#20-summary
chunk_level: summary
chunk_type: prose
heading: What's next
token_count: 106
summary: You can check that the Pods managed by the Deployment are still running: ``` `kubectl get pods -l app=nginx ` ``` ## What's next * Learn about [owners and...
---

You can check that the Pods managed by the Deployment are still running:
```
`kubectl get pods -l app=nginx
`
```
## What's next
* Learn about [owners and dependents](/docs/concepts/overview/working-with-objects/owners-dependents/) in Kubernetes.
* Learn about Kubernetes [finalizers](/docs/concepts/overview/working-with-objects/finalizers/).
* Learn about [garbage collection](/docs/concepts/architecture/garbage-collection/).