---
doc_id: ref/docs-reference-networking-service-protocols.md/docs-reference-networking-service-protocols
chunk_id: ref/docs-reference-networking-service-protocols.md/docs-reference-networking-service-protocols#4-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 120
summary: If your cloud provider supports it, you can use a Service in LoadBalancer mode to configure a load balancer outside of your Kubernetes cluster, in a special mode where your cloud provider's load...
---

If your cloud provider supports it, you can use a Service in LoadBalancer mode to
configure a load balancer outside of your Kubernetes cluster, in a special mode
where your cloud provider's load balancer implements HTTP / HTTPS reverse proxying,
with traffic forwarded to the backend endpoints for that Service.
Typically, you set the protocol for the Service to `TCP` and add an
[annotation](/docs/concepts/overview/working-with-objects/annotations)
(usually specific to your cloud provider) that configures the load balancer
to handle traffic at the HTTP level.