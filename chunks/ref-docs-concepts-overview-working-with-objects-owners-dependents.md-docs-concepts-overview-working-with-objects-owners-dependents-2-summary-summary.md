---
doc_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents
chunk_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents#2-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 128
summary: In Kubernetes, some [objects](/docs/concepts/overview/working-with-objects/#kubernetes-objects) are *owners* of other objects. For example, a...
---

In Kubernetes, some [objects](/docs/concepts/overview/working-with-objects/#kubernetes-objects) are
*owners* of other objects. For example, a
[ReplicaSet](/docs/concepts/workloads/controllers/replicaset/) is the owner
of a set of Pods. These owned objects are *dependents* of their owner.
Ownership is different from the [labels and selectors](/docs/concepts/overview/working-with-objects/labels/)
mechanism that some resources also use. For example, consider a Service that
creates `EndpointSlice` objects. The Service uses