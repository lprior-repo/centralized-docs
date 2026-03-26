---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#30-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 122
summary: * the Endpoints resource has a `endpointslice.kubernetes.io/skip-mirror` label set to `true`. * the Endpoints resource has a `control-plane.alpha.kubernetes.io/leader` annotation. * the corresponding...
---

* the Endpoints resource has a `endpointslice.kubernetes.io/skip-mirror` label
set to `true`.
* the Endpoints resource has a `control-plane.alpha.kubernetes.io/leader`
annotation.
* the corresponding Service resource does not exist.
* the corresponding Service resource has a non-nil selector.
Individual Endpoints resources may translate into multiple EndpointSlices. This
will occur if an Endpoints resource has multiple subsets or includes endpoints
with multiple IP families (IPv4 and IPv6). A maximum of 1000 addresses per
subset will be mirrored to EndpointSlices.