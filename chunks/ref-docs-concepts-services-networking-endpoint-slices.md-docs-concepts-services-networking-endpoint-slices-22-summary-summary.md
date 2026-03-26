---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#22-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 122
summary: 3. If there's still new endpoints left to add, try to fit them into a previously unchanged slice and/or create new ones. Importantly, the third step prioritizes limiting EndpointSlice updates over a...
---

3. If there's still new endpoints left to add, try to fit them into a previously
unchanged slice and/or create new ones.
Importantly, the third step prioritizes limiting EndpointSlice updates over a
perfectly full distribution of EndpointSlices. As an example, if there are 10
new endpoints to add and 2 EndpointSlices with room for 5 more endpoints each,
this approach will create a new EndpointSlice instead of filling up the 2
existing EndpointSlices. In other words, a single EndpointSlice creation is
preferable to multiple EndpointSlice updates.