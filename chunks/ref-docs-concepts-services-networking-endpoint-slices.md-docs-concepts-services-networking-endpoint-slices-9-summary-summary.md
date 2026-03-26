---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#9-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 104
summary: By default, the control plane creates and manages EndpointSlices to have no more than 100 endpoints each. You can configure this with the `--max-endpoints-per-slice`...
---

By default, the control plane creates and manages EndpointSlices to have no
more than 100 endpoints each. You can configure this with the
`--max-endpoints-per-slice`
[kube-controller-manager](/docs/reference/command-line-tools-reference/kube-controller-manager/)
flag, up to a maximum of 1000.
EndpointSlices act as the source of truth for
[kube-proxy](/docs/reference/command-line-tools-reference/kube-proxy/) when it comes to
how to route internal traffic.