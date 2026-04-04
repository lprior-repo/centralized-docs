---
doc_id: ref/docs-tasks-run-application-force-delete-stateful-set-pod.md/docs-tasks-run-application-force-delete-stateful-set-pod
chunk_id: ref/docs-tasks-run-application-force-delete-stateful-set-pod.md/docs-tasks-run-application-force-delete-stateful-set-pod#15-summary
chunk_level: summary
chunk_type: prose
heading: Delete Pods
token_count: 112
summary: will violate the at most one semantics that StatefulSet is designed to guarantee. When you force delete a StatefulSet pod, you are asserting that the Pod in question will never again make contact...
---

will violate the at most one semantics that StatefulSet is designed to guarantee.
When you force delete a StatefulSet pod, you are asserting that the Pod in question will never
again make contact with other Pods in the StatefulSet and its name can be safely freed up for a
replacement to be created.
If you want to delete a Pod forcibly using kubectl version &gt;= 1.5, do the following:
```
`kubectl delete pods &lt;pod&gt; --grace-period=0 --force
`
```