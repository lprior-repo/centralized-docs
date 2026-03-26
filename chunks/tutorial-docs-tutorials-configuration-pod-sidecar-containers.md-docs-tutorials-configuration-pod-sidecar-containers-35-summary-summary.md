---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#35-summary
chunk_level: summary
chunk_type: prose
heading: Adopting built-in sidecar containers
token_count: 87
summary: * If the file indicates that the `NativeSidecar` is running, it either does nothing and sleeps forever (in the case when Pod’s `restartPolicy=Always`) or exits immediately with exit code `0` (in the...
---

* If the file indicates that the `NativeSidecar` is running, it either does nothing and sleeps
forever (in the case when Pod’s `restartPolicy=Always`) or exits immediately with exit code `0`
(in the case when Pod’s `restartPolicy!=Always`).## What's next
* Learn more about [sidecar containers](/docs/concepts/workloads/pods/sidecar-containers/).