---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#20-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 86
summary: ### Distribution of EndpointSlices Each EndpointSlice has a set of ports that applies to all endpoints within the resource. When named ports are used for a Service, Pods may end up with different...
---

### Distribution of EndpointSlices
Each EndpointSlice has a set of ports that applies to all endpoints within the
resource. When named ports are used for a Service, Pods may end up with
different target port numbers for the same named port, requiring different
EndpointSlices.
The control plane tries to fill EndpointSlices as full as possible, but does not
actively rebalance them. The logic is fairly straightforward: