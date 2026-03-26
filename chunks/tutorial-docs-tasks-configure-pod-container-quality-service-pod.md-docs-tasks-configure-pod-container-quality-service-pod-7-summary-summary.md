---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#7-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 75
summary: #### Note: Kubernetes assigns the QoS class when the Pod is created, and it remains unchanged for the lifetime of the Pod. If you attempt to [resize the Pod's...
---

#### Note:
Kubernetes assigns the QoS class when the Pod is created, and it remains unchanged
for the lifetime of the Pod. If you attempt to
[resize the Pod's resources](/docs/tasks/configure-pod-container/resize-container-resources/)
to values that would result in a different QoS class, control plane rejects your request with an error message.