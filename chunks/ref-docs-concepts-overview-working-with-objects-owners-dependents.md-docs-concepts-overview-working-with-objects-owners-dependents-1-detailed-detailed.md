---
doc_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents
chunk_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Owner references in object specifications
token_count: 739
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
## Owner references in object specifications
Dependent objects have a `metadata.ownerReferences` field that references their
owner object. A valid owner reference consists of the object name and a [UID](/docs/concepts/overview/working-with-objects/names)
within the same [namespace](/docs/concepts/overview/working-with-objects/namespaces) as the dependent object. Kubernetes sets the value of
this field automatically for objects that are dependents of other objects like
ReplicaSets, DaemonSets, Deployments, Jobs and CronJobs, and ReplicationControllers.
You can also configure these relationships manually by changing the value of
this field. However, you usually don't need to and can allow Kubernetes to
automatically manage the relationships.
Dependent objects also have an `ownerReferences.blockOwnerDeletion` field that
takes a boolean value and controls whether specific dependents can block garbage
collection from deleting their owner object. Kubernetes automatically sets this
field to `true` if a [controller](/docs/concepts/architecture/controller/)
(for example, the Deployment controller) sets the value of the
`metadata.ownerReferences` field. You can also set the value of the
`blockOwnerDeletion` field manually to control which dependents block garbage
collection.
A Kubernetes admission controller controls user access to change this field for
dependent resources, based on the delete permissions of the owner. This control
prevents unauthorized users from delaying owner object deletion.
#### Note:
Cross-namespace owner references are disallowed by design.
Namespaced dependents can specify cluster-scoped or namespaced owners.
A namespaced owner **must** exist in the same namespace as the dependent.
If it does not, the owner reference is treated as absent, and the dependent
is subject to deletion once all owners are verified absent.
Cluster-scoped dependents can only specify cluster-scoped owners.
In v1.20+, if a cluster-scoped dependent specifies a namespaced kind as an owner,
it is treated as having an unresolvable owner reference, and is not able to be garbage collected.
In v1.20+, if the garbage collector detects an invalid cross-namespace `ownerReference`,
or a cluster-scoped dependent with an `ownerReference` referencing a namespaced kind, a warning Event
with a reason of `OwnerRefInvalidNamespace` and an `involvedObject` of the invalid dependent is reported.
You can check for that kind of Event by running
`kubectl get events -A --field-selector=reason=OwnerRefInvalidNamespace`.