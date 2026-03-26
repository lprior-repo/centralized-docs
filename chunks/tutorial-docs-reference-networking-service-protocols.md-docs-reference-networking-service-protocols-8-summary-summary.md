---
doc_id: tutorial/docs-reference-networking-service-protocols.md/docs-reference-networking-service-protocols
chunk_id: tutorial/docs-reference-networking-service-protocols.md/docs-reference-networking-service-protocols#8-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 125
summary: If your cloud provider supports it, you can use a Service set to `type: LoadBalancer` as a way to set up external reverse proxying, where the connection from client to load balancer is TLS encrypted...
---

If your cloud provider supports it, you can use a Service set to `type: LoadBalancer` as
a way to set up external reverse proxying, where the connection from client to load
balancer is TLS encrypted and the load balancer is the TLS server peer.
The connection from the load balancer to your workload can also be TLS,
or might be plain text. The exact options available to you depend on your
cloud provider or custom Service implementation.
Typically, you set the protocol to `TCP` and set an annotation
(usually specific to your cloud provider) that configures the load balancer