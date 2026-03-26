---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#11-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 99
summary: ### Conditions The EndpointSlice API stores conditions about endpoints that may be useful for consumers. The three conditions are `serving`, `terminating`, and `ready`. #### Serving FEATURE STATE:...
---

### Conditions
The EndpointSlice API stores conditions about endpoints that may be useful for consumers.
The three conditions are `serving`, `terminating`, and `ready`.
#### Serving
FEATURE STATE:
`Kubernetes v1.26 [stable]`
The `serving` condition indicates that the endpoint is currently serving responses, and
so it should be used as a target for Service traffic. For endpoints backed by a Pod, this
maps to the Pod's `Ready` condition.