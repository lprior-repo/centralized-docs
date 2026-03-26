---
doc_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods
chunk_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods#7-summary
chunk_level: summary
chunk_type: prose
heading: Diagnosing the problem
token_count: 50
summary: ``` `kubectl describe pods ${POD\_NAME} ` ``` Look at the state of the containers in the pod. Are they all `Running`? Have there been recent restarts? Continue debugging depending on the state of the...
---

```
`kubectl describe pods ${POD\_NAME}
`
```
Look at the state of the containers in the pod. Are they all `Running`?
Have there been recent restarts?
Continue debugging depending on the state of the pods.