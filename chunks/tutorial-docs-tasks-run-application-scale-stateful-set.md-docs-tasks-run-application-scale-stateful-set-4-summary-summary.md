---
doc_id: tutorial/docs-tasks-run-application-scale-stateful-set.md/docs-tasks-run-application-scale-stateful-set
chunk_id: tutorial/docs-tasks-run-application-scale-stateful-set.md/docs-tasks-run-application-scale-stateful-set#4-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 112
summary: ### Make in-place updates on your StatefulSets Alternatively, you can do [in-place updates](/docs/concepts/cluster-administration/manage-deployment/#in-place-updates-of-resources) on your...
---

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