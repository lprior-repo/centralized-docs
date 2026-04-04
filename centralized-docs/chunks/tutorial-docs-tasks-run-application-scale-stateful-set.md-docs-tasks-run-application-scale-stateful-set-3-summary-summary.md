---
doc_id: tutorial/docs-tasks-run-application-scale-stateful-set.md/docs-tasks-run-application-scale-stateful-set
chunk_id: tutorial/docs-tasks-run-application-scale-stateful-set.md/docs-tasks-run-application-scale-stateful-set#3-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 84
summary: ### Use kubectl to scale StatefulSets First, find the StatefulSet you want to scale. ``` `kubectl get statefulsets &lt;stateful-set-name&gt; ` ``` Change the number of replicas of your StatefulSet:...
---

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