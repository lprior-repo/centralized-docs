---
doc_id: tutorial/docs-tasks-configure-pod-container-attach-handler-lifecycle-event.md/docs-tasks-configure-pod-container-attach-handler-lifecycle-event
chunk_id: tutorial/docs-tasks-configure-pod-container-attach-handler-lifecycle-event.md/docs-tasks-configure-pod-container-attach-handler-lifecycle-event#10-summary
chunk_level: summary
chunk_type: prose
heading: Discussion
token_count: 118
summary: Kubernetes' management of the Container blocks until the preStop handler completes, unless the Pod's grace period expires. For more details, see [Pod...
---

Kubernetes' management of the Container blocks until the preStop handler completes,
unless the Pod's grace period expires. For more details, see
[Pod Lifecycle](/docs/concepts/workloads/pods/pod-lifecycle/).
#### Note:
Kubernetes only sends the preStop event when a Pod or a container in the Pod is *terminated*.
This means that the preStop hook is not invoked when the Pod is *completed*.
About this limitation, please see [Container hooks](/docs/concepts/containers/container-lifecycle-hooks/#container-hooks) for the detail.