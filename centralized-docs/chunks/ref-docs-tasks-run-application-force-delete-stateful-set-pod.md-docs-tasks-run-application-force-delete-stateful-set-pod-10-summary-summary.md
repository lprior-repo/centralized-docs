---
doc_id: ref/docs-tasks-run-application-force-delete-stateful-set-pod.md/docs-tasks-run-application-force-delete-stateful-set-pod
chunk_id: ref/docs-tasks-run-application-force-delete-stateful-set-pod.md/docs-tasks-run-application-force-delete-stateful-set-pod#10-summary
chunk_level: summary
chunk_type: prose
heading: Delete Pods
token_count: 56
summary: * The Node object is deleted (either by you, or by the [Node Controller](/docs/concepts/architecture/nodes/#node-controller)). * The kubelet on the unresponsive Node starts responding, kills the Pod...
---

* The Node object is deleted (either by you, or by the
[Node Controller](/docs/concepts/architecture/nodes/#node-controller)).
* The kubelet on the unresponsive Node starts responding, kills the Pod and removes the entry
from the apiserver.