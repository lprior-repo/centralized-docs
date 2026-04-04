---
doc_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation
chunk_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation#15-summary
chunk_level: summary
chunk_type: prose
heading: How can you avoid Service ClusterIP conflicts?
token_count: 128
summary: This example uses the IP address range: 10.96.0.0/20 (CIDR notation) for the IP addresses of Services. Range Size: 212 - 2 = 4094 Band Offset: `min(max(16, 4096/16), 256)` = `min(256, 256)` = 256...
---

This example uses the IP address range: 10.96.0.0/20 (CIDR notation) for the IP addresses
of Services.
Range Size: 212 - 2 = 4094
Band Offset: `min(max(16, 4096/16), 256)` = `min(256, 256)` = 256
Static band start: 10.96.0.1
Static band end: 10.96.1.0
Range end: 10.96.15.254
pie showData
title 10.96.0.0/20