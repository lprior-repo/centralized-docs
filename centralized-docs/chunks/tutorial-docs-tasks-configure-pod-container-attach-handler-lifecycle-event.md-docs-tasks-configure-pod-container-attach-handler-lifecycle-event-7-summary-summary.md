---
doc_id: tutorial/docs-tasks-configure-pod-container-attach-handler-lifecycle-event.md/docs-tasks-configure-pod-container-attach-handler-lifecycle-event
chunk_id: tutorial/docs-tasks-configure-pod-container-attach-handler-lifecycle-event.md/docs-tasks-configure-pod-container-attach-handler-lifecycle-event#7-summary
chunk_level: summary
chunk_type: prose
heading: Define postStart and preStop handlers
token_count: 102
summary: ``` `kubectl get pod lifecycle-demo ` ``` Get a shell into the Container running in your Pod: ``` `kubectl exec -it lifecycle-demo -- /bin/bash ` ``` In your shell, verify that the `postStart`...
---

```
`kubectl get pod lifecycle-demo
`
```
Get a shell into the Container running in your Pod:
```
`kubectl exec -it lifecycle-demo -- /bin/bash
`
```
In your shell, verify that the `postStart` handler created the `message` file:
```
`root@lifecycle-demo:/# cat /usr/share/message
`
```
The output shows the text written by the postStart handler:
```
`Hello from the postStart handler
`
```