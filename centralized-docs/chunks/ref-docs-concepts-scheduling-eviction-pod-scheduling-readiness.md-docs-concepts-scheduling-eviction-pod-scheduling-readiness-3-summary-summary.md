---
doc_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness
chunk_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness#3-summary
chunk_level: summary
chunk_type: prose
heading: Configuring Pod schedulingGates
token_count: 82
summary: The `schedulingGates` field contains a list of strings, and each string literal is perceived as a criteria that Pod should be satisfied before considered schedulable. This field can be initialized...
---

The `schedulingGates` field contains a list of strings, and each string literal is perceived as a
criteria that Pod should be satisfied before considered schedulable. This field can be initialized
only when a Pod is created (either by the client, or mutated during admission). After creation,
each schedulingGate can be removed in arbitrary order, but addition of a new scheduling gate is disallowed.