---
doc_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation
chunk_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation#18-summary
chunk_level: summary
chunk_type: prose
heading: How can you avoid Service ClusterIP conflicts?
token_count: 117
summary: This example uses the IP address range: 10.96.0.0/16 (CIDR notation) for the IP addresses of Services. Range Size: 216 - 2 = 65534 Band Offset: `min(max(16, 65536/16), 256)` = `min(4096, 256)` = 256...
---

This example uses the IP address range: 10.96.0.0/16 (CIDR notation) for the IP addresses
of Services.
Range Size: 216 - 2 = 65534
Band Offset: `min(max(16, 65536/16), 256)` = `min(4096, 256)` = 256
Static band start: 10.96.0.1
Static band ends: 10.96.1.0
Range end: 10.96.255.254
pie showData