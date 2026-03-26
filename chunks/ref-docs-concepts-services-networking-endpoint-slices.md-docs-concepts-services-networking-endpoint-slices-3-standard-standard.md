---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#3-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 324
summary: ### Topology information Each endpoint within an EndpointSlice can contain relevant topology information. The topology information includes the location of the endpoint and information about the...
---

### Topology information
Each endpoint within an EndpointSlice can contain relevant topology information.
The topology information includes the location of the endpoint and information
about the corresponding Node and zone. These are available in the following
per endpoint fields on EndpointSlices:
* `nodeName` - The name of the Node this endpoint is on.
* `zone` - The zone this endpoint is in.### Management
Most often, the control plane (specifically, the endpoint slice
[controller](/docs/concepts/architecture/controller/)) creates and
manages EndpointSlice objects. There are a variety of other use cases for
EndpointSlices, such as service mesh implementations, that could result in other
entities or controllers managing additional sets of EndpointSlices.
To ensure that multiple entities can manage EndpointSlices without interfering
with each other, Kubernetes defines the
[label](/docs/concepts/overview/working-with-objects/labels)
`endpointslice.kubernetes.io/managed-by`, which indicates the entity managing
an EndpointSlice.
The endpoint slice controller sets `endpointslice-controller.k8s.io` as the value
for this label on all EndpointSlices it manages. Other entities managing
EndpointSlices should also set a unique value for this label.
### Ownership
In most use cases, EndpointSlices are owned by the Service that the endpoint
slice object tracks endpoints for. This ownership is indicated by an owner
reference on each EndpointSlice as well as a `kubernetes.io/service-name`
label that enables simple lookups of all EndpointSlices belonging to a Service.