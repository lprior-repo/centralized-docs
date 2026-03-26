---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#23-summary
chunk_level: summary
chunk_type: prose
heading: Adopting built-in sidecar containers
token_count: 111
summary: The feature can be enabled on nodes with the version 1.28. The behavior of built-in sidecar container termination was different in version 1.28, and it is not recommended to adjust the behavior of a...
---

The feature can be enabled on nodes with the version 1.28. The behavior of built-in sidecar
container termination was different in version 1.28, and it is not recommended to adjust
the behavior of a sidecar to that behavior. However, if the only concern is the startup order, the
above statement can be changed to Nodes running version 1.28 with the feature gate enabled.
You should ensure that the feature gate is enabled for the API server(s) within the control plane
**and** for all nodes.