---
doc_id: ref/docs-reference-networking-service-protocols.md/docs-reference-networking-service-protocols
chunk_id: ref/docs-reference-networking-service-protocols.md/docs-reference-networking-service-protocols#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 481
summary: ### `TCP` You can use TCP for any kind of Service, and it's the default network protocol. ### `UDP` You can use UDP for most Services. For `type: LoadBalancer` Services, UDP support depends on the...
---

### `TCP`
You can use TCP for any kind of Service, and it's the default network protocol.
### `UDP`
You can use UDP for most Services. For `type: LoadBalancer` Services,
UDP support depends on the cloud provider offering this facility.
### HTTP
If your cloud provider supports it, you can use a Service in LoadBalancer mode to
configure a load balancer outside of your Kubernetes cluster, in a special mode
where your cloud provider's load balancer implements HTTP / HTTPS reverse proxying,
with traffic forwarded to the backend endpoints for that Service.
Typically, you set the protocol for the Service to `TCP` and add an
[annotation](/docs/concepts/overview/working-with-objects/annotations)
(usually specific to your cloud provider) that configures the load balancer
to handle traffic at the HTTP level.
This configuration might also include serving HTTPS (HTTP over TLS) and
reverse-proxying plain HTTP to your workload.
#### Note:
You can also use an [Ingress](/docs/concepts/services-networking/ingress/) to expose
HTTP/HTTPS Services.
You might additionally want to specify that the
[application protocol](/docs/concepts/services-networking/service/#application-protocol)
of the connection is `http` or `https`. Use `http` if the session from the
load balancer to your workload is HTTP without TLS, and use `https` if the
session from the load balancer to your workload uses TLS encryption.
### TLS
If your cloud provider supports it, you can use a Service set to `type: LoadBalancer` as
a way to set up external reverse proxying, where the connection from client to load
balancer is TLS encrypted and the load balancer is the TLS server peer.
The connection from the load balancer to your workload can also be TLS,
or might be plain text. The exact options available to you depend on your
cloud provider or custom Service implementation.
Typically, you set the protocol to `TCP` and set an annotation
(usually specific to your cloud provider) that configures the load balancer
to act as a TLS server. You would configure the TLS identity (as server,
and possibly also as a client that connects to your workload) using
mechanisms that are specific to your cloud provider.