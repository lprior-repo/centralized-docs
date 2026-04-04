---
doc_id: tutorial/docs-tasks-run-application-scale-stateful-set.md/docs-tasks-run-application-scale-stateful-set
chunk_id: tutorial/docs-tasks-run-application-scale-stateful-set.md/docs-tasks-run-application-scale-stateful-set#2-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 382
summary: ## Before you begin * StatefulSets are only available in Kubernetes version 1.5 or later. To check your version of Kubernetes, run `kubectl version`. * Not all stateful applications scale nicely. If...
---

## Before you begin
* StatefulSets are only available in Kubernetes version 1.5 or later.
To check your version of Kubernetes, run `kubectl version`.
* Not all stateful applications scale nicely. If you are unsure about whether
to scale your StatefulSets, see [StatefulSet concepts](/docs/concepts/workloads/controllers/statefulset/)
or [StatefulSet tutorial](/docs/tutorials/stateful-application/basic-stateful-set/) for further information.
* You should perform scaling only when you are confident that your stateful application
cluster is completely healthy.
### Use kubectl to scale StatefulSets
First, find the StatefulSet you want to scale.
```
`kubectl get statefulsets &lt;stateful-set-name&gt;
`
```
Change the number of replicas of your StatefulSet:
```
`kubectl scale statefulsets &lt;stateful-set-name&gt; --replicas=&lt;new-replicas&gt;
`
```
### Make in-place updates on your StatefulSets
Alternatively, you can do
[in-place updates](/docs/concepts/cluster-administration/manage-deployment/#in-place-updates-of-resources)
on your StatefulSets.
If your StatefulSet was initially created with `kubectl apply`,
update `.spec.replicas` of the StatefulSet manifests, and then do a `kubectl apply`:
```
`kubectl apply -f &lt;stateful-set-file-updated&gt;
`
```
Otherwise, edit that field with `kubectl edit`:
```
`kubectl edit statefulsets &lt;stateful-set-name&gt;
`
```
Or use `kubectl patch`:
```
`kubectl patch statefulsets &lt;stateful-set-name&gt; -p '{"spec":{"replicas":&lt;new-replicas&gt;}}'
`
```