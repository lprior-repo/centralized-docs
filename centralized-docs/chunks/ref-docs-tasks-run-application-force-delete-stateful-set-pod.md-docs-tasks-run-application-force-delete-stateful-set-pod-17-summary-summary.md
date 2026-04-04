---
doc_id: ref/docs-tasks-run-application-force-delete-stateful-set-pod.md/docs-tasks-run-application-force-delete-stateful-set-pod
chunk_id: ref/docs-tasks-run-application-force-delete-stateful-set-pod.md/docs-tasks-run-application-force-delete-stateful-set-pod#17-summary
chunk_level: summary
chunk_type: prose
heading: Delete Pods
token_count: 73
summary: If even after these commands the pod is stuck on `Unknown` state, use the following command to remove the pod from the cluster: ``` `kubectl patch pod &lt;pod&gt; -p...
---

If even after these commands the pod is stuck on `Unknown` state, use the following command to
remove the pod from the cluster:
```
`kubectl patch pod &lt;pod&gt; -p '{"metadata":{"finalizers":null}}'
`
```
Always perform force deletion of StatefulSet Pods carefully and with complete knowledge of the risks involved.