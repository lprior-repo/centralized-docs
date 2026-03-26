---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#21-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 51
summary: 1. Iterate through existing EndpointSlices, remove endpoints that are no longer desired and update matching endpoints that have changed. 2. Iterate through EndpointSlices that have been modified in...
---

1. Iterate through existing EndpointSlices, remove endpoints that are no longer
desired and update matching endpoints that have changed.
2. Iterate through EndpointSlices that have been modified in the first step and
fill them up with any new endpoints needed.