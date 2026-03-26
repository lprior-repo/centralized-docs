---
doc_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods
chunk_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods#23-summary
chunk_level: summary
chunk_type: prose
heading: Diagnosing the problem
token_count: 77
summary: ### Debugging Replication Controllers Replication controllers are fairly straightforward. They can either create Pods or they can't. If they can't create pods, then please refer to the [instructions...
---

### Debugging Replication Controllers
Replication controllers are fairly straightforward. They can either create Pods or they can't.
If they can't create pods, then please refer to the
[instructions above](#debugging-pods) to debug your pods.
You can also use `kubectl describe rc ${CONTROLLER\_NAME}` to introspect events
related to the replication controller.