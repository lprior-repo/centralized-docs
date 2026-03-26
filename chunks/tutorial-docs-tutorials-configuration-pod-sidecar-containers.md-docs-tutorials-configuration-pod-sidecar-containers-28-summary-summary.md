---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#28-summary
chunk_level: summary
chunk_type: prose
heading: Adopting built-in sidecar containers
token_count: 117
summary: The way to check this is to use the `kubectl describe pod` command with your Pod that has passed through mutating admission. If any tools stripped out the new field (`restartPolicy:Always`), you will...
---

The way to check this is to use the `kubectl describe pod` command with your Pod that has passed through
mutating admission. If any tools stripped out the new field (`restartPolicy:Always`),
you will not see it in the command output.
If you hit an issue like this, please advise the author of the tools or the webhooks
use one of the patching strategies for modifying objects instead of a full object update.
#### Note
Mutating webhook may update Pods based on some conditions.
Thus, sidecar containers may work for some Pods and fail for others.