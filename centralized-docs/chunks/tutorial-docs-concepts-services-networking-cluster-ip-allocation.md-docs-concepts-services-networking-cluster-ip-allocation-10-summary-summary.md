---
doc_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation
chunk_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation#10-summary
chunk_level: summary
chunk_type: prose
heading: How can you avoid Service ClusterIP conflicts?
token_count: 125
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