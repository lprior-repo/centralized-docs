---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#17-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 124
summary: * `zone` - The zone this endpoint is in.### Management Most often, the control plane (specifically, the endpoint slice [controller](/docs/concepts/architecture/controller/)) creates and manages...
---

* `zone` - The zone this endpoint is in.### Management
Most often, the control plane (specifically, the endpoint slice
[controller](/docs/concepts/architecture/controller/)) creates and
manages EndpointSlice objects. There are a variety of other use cases for
EndpointSlices, such as service mesh implementations, that could result in other
entities or controllers managing additional sets of EndpointSlices.
To ensure that multiple entities can manage EndpointSlices without interfering
with each other, Kubernetes defines the
[label](/docs/concepts/overview/working-with-objects/labels)