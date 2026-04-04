---
doc_id: ref/docs-tasks-run-application-force-delete-stateful-set-pod.md/docs-tasks-run-application-force-delete-stateful-set-pod
chunk_id: ref/docs-tasks-run-application-force-delete-stateful-set-pod.md/docs-tasks-run-application-force-delete-stateful-set-pod#3-standard
chunk_level: standard
chunk_type: prose
heading: Delete Pods
token_count: 339
summary: ### Force Deletion Force deletions **do not** wait for confirmation from the kubelet that the Pod has been terminated. Irrespective of whether a force deletion is successful in killing a Pod, it will...
---

### Force Deletion
Force deletions **do not** wait for confirmation from the kubelet that the Pod has been terminated.
Irrespective of whether a force deletion is successful in killing a Pod, it will immediately
free up the name from the apiserver. This would let the StatefulSet controller create a replacement
Pod with that same identity; this can lead to the duplication of a still-running Pod,
and if said Pod can still communicate with the other members of the StatefulSet,
will violate the at most one semantics that StatefulSet is designed to guarantee.
When you force delete a StatefulSet pod, you are asserting that the Pod in question will never
again make contact with other Pods in the StatefulSet and its name can be safely freed up for a
replacement to be created.
If you want to delete a Pod forcibly using kubectl version &gt;= 1.5, do the following:
```
`kubectl delete pods &lt;pod&gt; --grace-period=0 --force
`
```
If you're using any version of kubectl &lt;= 1.4, you should omit the `--force` option and use:
```
`kubectl delete pods &lt;pod&gt; --grace-period=0
`
```
If even after these commands the pod is stuck on `Unknown` state, use the following command to
remove the pod from the cluster:
```
`kubectl patch pod &lt;pod&gt; -p '{"metadata":{"finalizers":null}}'
`
```
Always perform force deletion of StatefulSet Pods carefully and with complete knowledge of the risks involved.