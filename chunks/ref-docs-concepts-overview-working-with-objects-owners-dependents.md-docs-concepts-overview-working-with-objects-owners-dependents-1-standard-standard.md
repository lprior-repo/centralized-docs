---
doc_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents
chunk_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 217
summary: # Owners and Dependents In Kubernetes, some [objects](/docs/concepts/overview/working-with-objects/#kubernetes-objects) are *owners* of other objects. For example, a...
---

# Owners and Dependents
In Kubernetes, some [objects](/docs/concepts/overview/working-with-objects/#kubernetes-objects) are
*owners* of other objects. For example, a
[ReplicaSet](/docs/concepts/workloads/controllers/replicaset/) is the owner
of a set of Pods. These owned objects are *dependents* of their owner.
Ownership is different from the [labels and selectors](/docs/concepts/overview/working-with-objects/labels/)
mechanism that some resources also use. For example, consider a Service that
creates `EndpointSlice` objects. The Service uses [labels](/docs/concepts/overview/working-with-objects/labels) to allow the control plane to
determine which `EndpointSlice` objects are used for that Service. In addition
to the labels, each `EndpointSlice` that is managed on behalf of a Service has
an owner reference. Owner references help different parts of Kubernetes avoid
interfering with objects they don’t control.