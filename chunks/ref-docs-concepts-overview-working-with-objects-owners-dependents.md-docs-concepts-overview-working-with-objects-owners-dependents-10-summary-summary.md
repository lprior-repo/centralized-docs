---
doc_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents
chunk_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents#10-summary
chunk_level: summary
chunk_type: prose
heading: Owner references in object specifications
token_count: 115
summary: it is treated as having an unresolvable owner reference, and is not able to be garbage collected. In v1.20+, if the garbage collector detects an invalid cross-namespace `ownerReference`, or a...
---

it is treated as having an unresolvable owner reference, and is not able to be garbage collected.
In v1.20+, if the garbage collector detects an invalid cross-namespace `ownerReference`,
or a cluster-scoped dependent with an `ownerReference` referencing a namespaced kind, a warning Event
with a reason of `OwnerRefInvalidNamespace` and an `involvedObject` of the invalid dependent is reported.
You can check for that kind of Event by running
`kubectl get events -A --field-selector=reason=OwnerRefInvalidNamespace`.