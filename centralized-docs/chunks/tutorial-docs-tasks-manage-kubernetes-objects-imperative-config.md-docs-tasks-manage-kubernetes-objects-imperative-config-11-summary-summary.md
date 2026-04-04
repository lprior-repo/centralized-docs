---
doc_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config
chunk_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config#11-summary
chunk_level: summary
chunk_type: prose
heading: How to create objects
token_count: 99
summary: #### Note: If configuration file has specified the `generateName` field in the `metadata` section instead of the `name` field, you cannot delete the object using `kubectl delete -f...
---

#### Note:
If configuration file has specified the `generateName` field in the `metadata`
section instead of the `name` field, you cannot delete the object using
`kubectl delete -f &lt;filename|url&gt;`.
You will have to use other flags for deleting the object. For example:
```
`kubectl delete &lt;type&gt; &lt;name&gt;
kubectl delete &lt;type&gt; -l &lt;label&gt;
`
```