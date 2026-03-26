---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#25-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 54
summary: ### Duplicate endpoints Due to the nature of EndpointSlice changes, endpoints may be represented in more than one EndpointSlice at the same time. This naturally occurs as changes to different...
---

### Duplicate endpoints
Due to the nature of EndpointSlice changes, endpoints may be represented in more
than one EndpointSlice at the same time. This naturally occurs as changes to
different EndpointSlice objects can arrive at the Kubernetes client watch / cache
at different times.