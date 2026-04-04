---
doc_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation
chunk_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation#12-summary
chunk_level: summary
chunk_type: prose
heading: How can you avoid Service ClusterIP conflicts?
token_count: 128
summary: This example uses the IP address range: 10.96.0.0/24 (CIDR notation) for the IP addresses of Services. Range Size: 28 - 2 = 254 Band Offset: `min(max(16, 256/16), 256)` = `min(16, 256)` = 16 Static...
---

This example uses the IP address range: 10.96.0.0/24 (CIDR notation) for the IP addresses
of Services.
Range Size: 28 - 2 = 254
Band Offset: `min(max(16, 256/16), 256)` = `min(16, 256)` = 16
Static band start: 10.96.0.1
Static band end: 10.96.0.16
Range end: 10.96.0.254
pie showData
title 10.96.0.0/24
"