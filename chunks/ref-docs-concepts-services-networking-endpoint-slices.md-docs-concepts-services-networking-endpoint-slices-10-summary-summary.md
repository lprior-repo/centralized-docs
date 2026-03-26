---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#10-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 73
summary: ### Address types EndpointSlices support two address types: * IPv4 * IPv6 Each `EndpointSlice` object represents a specific IP address type. If you have a Service that is available via IPv4 and IPv6,...
---

### Address types
EndpointSlices support two address types:
* IPv4
* IPv6
Each `EndpointSlice` object represents a specific IP address type. If you have
a Service that is available via IPv4 and IPv6, there will be at least two
`EndpointSlice` objects (one for IPv4, and one for IPv6).