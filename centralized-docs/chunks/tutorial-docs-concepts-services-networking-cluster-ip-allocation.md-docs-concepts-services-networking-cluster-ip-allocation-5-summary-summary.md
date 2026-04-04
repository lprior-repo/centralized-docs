---
doc_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation
chunk_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation#5-summary
chunk_level: summary
chunk_type: prose
heading: How Service ClusterIPs are allocated?
token_count: 119
summary: ## How Service ClusterIPs are allocated? When Kubernetes needs to assign a virtual IP address for a Service, that assignment happens one of two ways: *dynamically*the cluster's control plane...
---

## How Service ClusterIPs are allocated?
When Kubernetes needs to assign a virtual IP address for a Service,
that assignment happens one of two ways:
*dynamically*the cluster's control plane automatically picks a free IP address from within the configured IP range for `type: ClusterIP` Services.*statically*you specify an IP address of your choice, from within the configured IP range for Services.
Across your whole cluster, every Service `ClusterIP` must be unique.
Trying to create a Service with a specific `ClusterIP` that has already
been allocated will return an error.