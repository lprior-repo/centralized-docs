---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#19-summary
chunk_level: summary
chunk_type: prose
heading: Adopting built-in sidecar containers
token_count: 111
summary: The `SidecarContainers` [feature gate](/docs/reference/command-line-tools-reference/feature-gates/) is in beta state starting from Kubernetes version 1.29 and is enabled by default. Some clusters may...
---

The `SidecarContainers` [feature gate](/docs/reference/command-line-tools-reference/feature-gates/)
is in beta state starting from Kubernetes version 1.29 and is enabled by default.
Some clusters may have this feature disabled or have software installed that is incompatible with the feature.
When this happens, the Pod may be rejected or the sidecar containers may block Pod startup,
rendering the Pod useless. This condition is easy to detect as the Pod simply gets stuck on
initialization. However, it is often unclear what caused the problem.