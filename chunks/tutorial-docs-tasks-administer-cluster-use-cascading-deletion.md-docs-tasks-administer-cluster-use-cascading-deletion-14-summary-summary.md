---
doc_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion
chunk_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion#14-summary
chunk_level: summary
chunk_type: prose
heading: Use background cascading deletion
token_count: 124
summary: 2. Use either `kubectl` or the Kubernetes API to delete the Deployment, depending on the Kubernetes version your cluster runs. To check the version, enter `kubectl version`. You can delete objects...
---

2. Use either `kubectl` or the Kubernetes API to delete the Deployment,
depending on the Kubernetes version your cluster runs.
To check the version, enter `kubectl version`.
You can delete objects using background cascading deletion using `kubectl`
or the Kubernetes API.
Kubernetes uses background cascading deletion by default, and does so
even if you run the following commands without the `--cascade` flag or the
`propagationPolicy` argument.
**Using kubectl**
Run the following command:
```
`kubectl delete deployment nginx-deployment --cascade=background
`
```
**Using the Kubernetes API**