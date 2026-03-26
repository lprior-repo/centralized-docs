---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#5-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 122
summary: The EndpointSlice API is the mechanism that Kubernetes uses to let your Service scale to handle large numbers of backends, and allows the cluster to update its list of healthy backends efficiently....
---

The EndpointSlice API is the mechanism that Kubernetes uses to let your Service scale to handle large numbers of backends, and allows the cluster to update its list of healthy backends efficiently.
FEATURE STATE:
`Kubernetes v1.21 [stable]`
EndpointSlices track the IP addresses of backend endpoints.
EndpointSlices are normally associated with a
[Service](/docs/concepts/services-networking/service/) and the backend endpoints typically represent
[Pods](/docs/concepts/workloads/pods/).## EndpointSlice API
In Kubernetes, an EndpointSlice contains references to a set of network