---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#18-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 92
summary: with each other, Kubernetes defines the [label](/docs/concepts/overview/working-with-objects/labels) `endpointslice.kubernetes.io/managed-by`, which indicates the entity managing an EndpointSlice....
---

with each other, Kubernetes defines the
[label](/docs/concepts/overview/working-with-objects/labels)
`endpointslice.kubernetes.io/managed-by`, which indicates the entity managing
an EndpointSlice.
The endpoint slice controller sets `endpointslice-controller.k8s.io` as the value
for this label on all EndpointSlices it manages. Other entities managing
EndpointSlices should also set a unique value for this label.