---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#28-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 125
summary: FEATURE STATE: `Kubernetes v1.33 [deprecated]` The EndpointSlice API is a replacement for the older Endpoints API. To preserve compatibility with older controllers and user workloads that expect...
---

FEATURE STATE:
`Kubernetes v1.33 [deprecated]`
The EndpointSlice API is a replacement for the older Endpoints API. To
preserve compatibility with older controllers and user workloads that
expect [kube-proxy](/docs/reference/command-line-tools-reference/kube-proxy/)
to route traffic based on Endpoints resources, the cluster's control
plane mirrors most user-created Endpoints resources to corresponding
EndpointSlices.
(However, this feature, like the rest of the Endpoints API, is
deprecated. Users who manually specify endpoints for selectorless
Services should do so by creating EndpointSlice resources directly,