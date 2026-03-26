---
doc_id: tutorial/docs-reference-networking-service-protocols.md/docs-reference-networking-service-protocols
chunk_id: tutorial/docs-reference-networking-service-protocols.md/docs-reference-networking-service-protocols#6-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 110
summary: #### Note: You can also use an [Ingress](/docs/concepts/services-networking/ingress/) to expose HTTP/HTTPS Services. You might additionally want to specify that the [application...
---

#### Note:
You can also use an [Ingress](/docs/concepts/services-networking/ingress/) to expose
HTTP/HTTPS Services.
You might additionally want to specify that the
[application protocol](/docs/concepts/services-networking/service/#application-protocol)
of the connection is `http` or `https`. Use `http` if the session from the
load balancer to your workload is HTTP without TLS, and use `https` if the
session from the load balancer to your workload uses TLS encryption.