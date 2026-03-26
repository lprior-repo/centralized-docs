---
doc_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents
chunk_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents#3-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 110
summary: mechanism that some resources also use. For example, consider a Service that creates `EndpointSlice` objects. The Service uses [labels](/docs/concepts/overview/working-with-objects/labels) to allow...
---

mechanism that some resources also use. For example, consider a Service that
creates `EndpointSlice` objects. The Service uses [labels](/docs/concepts/overview/working-with-objects/labels) to allow the control plane to
determine which `EndpointSlice` objects are used for that Service. In addition
to the labels, each `EndpointSlice` that is managed on behalf of a Service has
an owner reference. Owner references help different parts of Kubernetes avoid
interfering with objects they don’t control.