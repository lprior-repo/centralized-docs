---
doc_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods
chunk_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods#13-summary
chunk_level: summary
chunk_type: prose
heading: Diagnosing the problem
token_count: 113
summary: * Try to manually pull the image to see if the image can be pulled. For example, if you use Docker on your PC, run `docker pull &lt;image&gt;`.#### My pod stays terminating If a Pod is stuck in the...
---

* Try to manually pull the image to see if the image can be pulled. For example,
if you use Docker on your PC, run `docker pull &lt;image&gt;`.#### My pod stays terminating
If a Pod is stuck in the `Terminating` state, it means that a deletion has been
issued for the Pod, but the control plane is unable to delete the Pod object.
This typically happens if the Pod has a [finalizer](/docs/concepts/overview/working-with-objects/finalizers/)
and there is an