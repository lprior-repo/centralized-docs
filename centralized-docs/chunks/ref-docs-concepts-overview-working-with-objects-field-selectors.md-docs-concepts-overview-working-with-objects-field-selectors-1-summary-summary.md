---
doc_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors
chunk_id: ref/docs-concepts-overview-working-with-objects-field-selectors.md/docs-concepts-overview-working-with-objects-field-selectors#1-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 114
summary: # Field Selectors *Field selectors* let you select Kubernetes [objects](/docs/concepts/overview/working-with-objects/#kubernetes-objects) based on the value of one or more resource fields. Here are...
---

# Field Selectors
*Field selectors* let you select Kubernetes [objects](/docs/concepts/overview/working-with-objects/#kubernetes-objects) based on the
value of one or more resource fields. Here are some examples of field selector queries:
* `metadata.name=my-service`
* `metadata.namespace!=default`
* `status.phase=Pending`
This `kubectl` command selects all Pods for which the value of the [`status.phase`](/docs/concepts/workloads/pods/pod-lifecycle/#pod-phase) field is `Running`: