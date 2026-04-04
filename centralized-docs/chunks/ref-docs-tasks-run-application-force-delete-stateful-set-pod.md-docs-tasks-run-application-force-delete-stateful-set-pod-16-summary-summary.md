---
doc_id: ref/docs-tasks-run-application-force-delete-stateful-set-pod.md/docs-tasks-run-application-force-delete-stateful-set-pod
chunk_id: ref/docs-tasks-run-application-force-delete-stateful-set-pod.md/docs-tasks-run-application-force-delete-stateful-set-pod#16-summary
chunk_level: summary
chunk_type: prose
heading: Delete Pods
token_count: 104
summary: ``` `kubectl delete pods &lt;pod&gt; --grace-period=0 --force ` ``` If you're using any version of kubectl &lt;= 1.4, you should omit the `--force` option and use: ``` `kubectl delete pods...
---

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