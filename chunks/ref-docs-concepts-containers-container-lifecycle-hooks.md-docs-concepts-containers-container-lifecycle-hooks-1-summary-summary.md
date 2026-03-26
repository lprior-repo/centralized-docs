---
doc_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks
chunk_id: ref/docs-concepts-containers-container-lifecycle-hooks.md/docs-concepts-containers-container-lifecycle-hooks#1-summary
chunk_level: summary
chunk_type: prose
heading: Overview
token_count: 90
summary: # Container Lifecycle Hooks This page describes how kubelet managed Containers can use the Container lifecycle hook framework to run code triggered by events during their management lifecycle. ##...
---

# Container Lifecycle Hooks
This page describes how kubelet managed Containers can use the Container lifecycle hook framework
to run code triggered by events during their management lifecycle.
## Overview
Analogous to many programming language frameworks that have component lifecycle hooks, such as Angular,
Kubernetes provides Containers with lifecycle hooks.
The hooks enable Containers to be aware of events in their management lifecycle
and run code implemented in a handler when the corresponding lifecycle hook is executed.