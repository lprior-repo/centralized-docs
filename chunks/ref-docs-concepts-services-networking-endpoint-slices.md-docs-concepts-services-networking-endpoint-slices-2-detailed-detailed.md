---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 871
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
### Distribution of EndpointSlices
Each EndpointSlice has a set of ports that applies to all endpoints within the
resource. When named ports are used for a Service, Pods may end up with
different target port numbers for the same named port, requiring different
EndpointSlices.
The control plane tries to fill EndpointSlices as full as possible, but does not
actively rebalance them. The logic is fairly straightforward:
1. Iterate through existing EndpointSlices, remove endpoints that are no longer
desired and update matching endpoints that have changed.
2. Iterate through EndpointSlices that have been modified in the first step and
fill them up with any new endpoints needed.
3. If there's still new endpoints left to add, try to fit them into a previously
unchanged slice and/or create new ones.
Importantly, the third step prioritizes limiting EndpointSlice updates over a
perfectly full distribution of EndpointSlices. As an example, if there are 10
new endpoints to add and 2 EndpointSlices with room for 5 more endpoints each,
this approach will create a new EndpointSlice instead of filling up the 2
existing EndpointSlices. In other words, a single EndpointSlice creation is
preferable to multiple EndpointSlice updates.
With kube-proxy running on each Node and watching EndpointSlices, every change
to an EndpointSlice becomes relatively expensive since it will be transmitted to
every Node in the cluster. This approach is intended to limit the number of
changes that need to be sent to every Node, even if it may result with multiple
EndpointSlices that are not full.
In practice, this less than ideal distribution should be rare. Most changes
processed by the EndpointSlice controller will be small enough to fit in an
existing EndpointSlice, and if not, a new EndpointSlice is likely going to be
necessary soon anyway. Rolling updates of Deployments also provide a natural
repacking of EndpointSlices with all Pods and their corresponding endpoints
getting replaced.
### Duplicate endpoints
Due to the nature of EndpointSlice changes, endpoints may be represented in more
than one EndpointSlice at the same time. This naturally occurs as changes to
different EndpointSlice objects can arrive at the Kubernetes client watch / cache
at different times.
#### Note:
Clients of the EndpointSlice API must iterate through all the existing EndpointSlices
associated to a Service and build a complete list of unique network endpoints. It is
important to mention that endpoints may be duplicated in different EndpointSlices.
You can find a reference implementation for how to perform this endpoint aggregation
and deduplication as part of the `EndpointSliceCache` code within `kube-proxy`.