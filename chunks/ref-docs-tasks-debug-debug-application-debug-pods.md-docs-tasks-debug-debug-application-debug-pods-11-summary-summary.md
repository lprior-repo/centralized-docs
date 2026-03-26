---
doc_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods
chunk_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods#11-summary
chunk_level: summary
chunk_type: prose
heading: Diagnosing the problem
token_count: 75
summary: #### My pod stays waiting If a Pod is stuck in the `Waiting` state, then it has been scheduled to a worker node, but it can't run on that machine. Again, the information from `kubectl describe ...`...
---

#### My pod stays waiting
If a Pod is stuck in the `Waiting` state, then it has been scheduled to a worker node,
but it can't run on that machine. Again, the information from `kubectl describe ...`
should be informative. The most common cause of `Waiting` pods is a failure to pull the image.
There are three things to check: