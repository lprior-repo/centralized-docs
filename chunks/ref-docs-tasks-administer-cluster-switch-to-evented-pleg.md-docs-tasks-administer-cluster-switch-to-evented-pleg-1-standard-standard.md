---
doc_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg
chunk_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 124
summary: # Switching from Polling to CRI Event-based Updates to Container Status FEATURE STATE: `Kubernetes v1.26 [alpha]`(disabled by default) This page shows how to migrate nodes to use event based updates...
---

# Switching from Polling to CRI Event-based Updates to Container Status
FEATURE STATE:
`Kubernetes v1.26 [alpha]`(disabled by default)
This page shows how to migrate nodes to use event based updates for container status. The event-based
implementation reduces node resource consumption by the kubelet, compared to the legacy approach
that relies on polling.
You may know this feature as *evented Pod lifecycle event generator (PLEG)*. That's the name used
internally within the Kubernetes project for a key implementation detail.
The polling based approach is referred to as *generic PLEG*.