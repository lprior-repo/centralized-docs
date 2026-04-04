---
doc_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation
chunk_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation#3-standard
chunk_level: standard
chunk_type: prose
heading: How can you avoid Service ClusterIP conflicts?
token_count: 419
summary: ## How can you avoid Service ClusterIP conflicts? The allocation strategy implemented in Kubernetes to allocate ClusterIPs to Services reduces the risk of collision. The `ClusterIP` range is divided,...
---

## How can you avoid Service ClusterIP conflicts?
The allocation strategy implemented in Kubernetes to allocate ClusterIPs to Services reduces the
risk of collision.
The `ClusterIP` range is divided, based on the formula `min(max(16, cidrSize / 16), 256)`,
described as *never less than 16 or more than 256 with a graduated step between them*.
Dynamic IP assignment uses the upper band by default, once this has been exhausted it will
use the lower range. This will allow users to use static allocations on the lower band with a low
risk of collision.
### Example 1
This example uses the IP address range: 10.96.0.0/24 (CIDR notation) for the IP addresses
of Services.
Range Size: 28 - 2 = 254
Band Offset: `min(max(16, 256/16), 256)` = `min(16, 256)` = 16
Static band start: 10.96.0.1
Static band end: 10.96.0.16
Range end: 10.96.0.254
pie showData
title 10.96.0.0/24
"Static" : 16
"Dynamic" : 238
### Example 2
This example uses the IP address range: 10.96.0.0/20 (CIDR notation) for the IP addresses
of Services.
Range Size: 212 - 2 = 4094
Band Offset: `min(max(16, 4096/16), 256)` = `min(256, 256)` = 256
Static band start: 10.96.0.1
Static band end: 10.96.1.0
Range end: 10.96.15.254
pie showData
title 10.96.0.0/20
"Static" : 256
"Dynamic" : 3838