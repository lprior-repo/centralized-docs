---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#5-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 128
summary: This section is relevant for people adopting a new built-in [sidecar containers](/docs/concepts/workloads/pods/sidecar-containers/) feature for their workloads. Sidecar container is not a new concept...
---

This section is relevant for people adopting a new built-in
[sidecar containers](/docs/concepts/workloads/pods/sidecar-containers/) feature for their workloads.
Sidecar container is not a new concept as posted in the
[blog post](/blog/2015/06/the-distributed-system-toolkit-patterns/).
Kubernetes allows running multiple containers in a Pod to implement this concept.
However, running a sidecar container as a regular container
has a lot of limitations being fixed with the new built-in sidecar containers support.
FEATURE STATE:
`Kubernetes v1.33 [stable]`