---
doc_id: tutorial/docs-tasks-configure-pod-container-attach-handler-lifecycle-event.md/docs-tasks-configure-pod-container-attach-handler-lifecycle-event
chunk_id: tutorial/docs-tasks-configure-pod-container-attach-handler-lifecycle-event.md/docs-tasks-configure-pod-container-attach-handler-lifecycle-event#9-summary
chunk_level: summary
chunk_type: prose
heading: Discussion
token_count: 127
summary: Kubernetes sends the postStart event immediately after the Container is created. There is no guarantee, however, that the postStart handler is called before the Container's entrypoint is called. The...
---

Kubernetes sends the postStart event immediately after the Container is created.
There is no guarantee, however, that the postStart handler is called before
the Container's entrypoint is called. The postStart handler runs asynchronously
relative to the Container's code, but Kubernetes' management of the container
blocks until the postStart handler completes. The Container's status is not
set to RUNNING until the postStart handler completes.
Kubernetes sends the preStop event immediately before the Container is terminated.
Kubernetes' management of the Container blocks until the preStop handler completes,
unless the Pod's grace period expires. For more details, see