---
doc_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation
chunk_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation#4-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 86
summary: # Service ClusterIP allocation In Kubernetes, [Services](/docs/concepts/services-networking/service/) are an abstract way to expose an application running on a set of Pods. Services can have a...
---

# Service ClusterIP allocation
In Kubernetes, [Services](/docs/concepts/services-networking/service/) are an abstract way to expose
an application running on a set of Pods. Services
can have a cluster-scoped virtual IP address (using a Service of `type: ClusterIP`).
Clients can connect using that virtual IP address, and Kubernetes then load-balances traffic to that
Service across the different backing Pods.