---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#21-summary
chunk_level: summary
chunk_type: prose
heading: Adopting built-in sidecar containers
token_count: 52
summary: ### Ensure the feature gate is enabled As a very first step, make sure that both API server and Nodes are at Kubernetes version v1.29 or later. The feature will break on clusters where Nodes are...
---

### Ensure the feature gate is enabled
As a very first step, make sure that both API server and Nodes are at Kubernetes version v1.29 or
later. The feature will break on clusters where Nodes are running earlier versions where it is not enabled.