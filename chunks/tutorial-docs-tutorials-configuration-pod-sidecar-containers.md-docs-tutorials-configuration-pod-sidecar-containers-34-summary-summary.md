---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#34-summary
chunk_level: summary
chunk_type: prose
heading: Adopting built-in sidecar containers
token_count: 71
summary: * Inject regular container, let's call it `OldWaySidecar`. * `OldWaySidecar` on start checks the presence of a file in an empty dir. * If the file indicates that the `NativeSidecar` is NOT running,...
---

* Inject regular container, let's call it `OldWaySidecar`.
* `OldWaySidecar` on start checks the presence of a file in an empty dir.
* If the file indicates that the `NativeSidecar` is NOT running, it assumes that the sidecar
feature is not supported and works assuming it is the sidecar.