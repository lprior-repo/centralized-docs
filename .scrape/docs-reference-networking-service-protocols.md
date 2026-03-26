---
url: https://kubernetes.io/docs/reference/networking/service-protocols/
title: service protocols
word_count: 496
filtered: true
elements_removed: 0
density_score: 0.88
---

## Table of Contents

    - [`SCTP`](#sctp)
      - [Support for multihomed SCTP associations](#support-for-multihomed-sctp-associations)
    - [`TCP`](#tcp)
    - [`UDP`](#udp)
    - [HTTP](#http)
      - [Note:](#note)
    - [TLS](#tls)
  - [Feedback](#feedback)

---

### `SCTP`
FEATURE STATE:
`Kubernetes v1.20 [stable]`
When using a network plugin that supports SCTP traffic, you can use SCTP for
most Services. For `type: LoadBalancer` Services, SCTP support depends on the cloud
provider offering this facility. (Most do not).
SCTP is not supported on nodes that run Windows.
#### Support for multihomed SCTP associations
The support of multihomed SCTP associations requires that the CNI plugin can support the assignment of multiple interfaces and IP addresses to a Pod.
NAT for multihomed SCTP associations requires special logic in the corresponding kernel modules.
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
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified December 05, 2022 at 10:36 AM PST: [Remove references to kube-proxy userspace mode (37ee1e335c)](https://github.com/kubernetes/website/commit/37ee1e335cde6bf42f8e47acf01f65d4325458c2)
## Related Pages

- [Secrets](docs-concepts-configuration-secret.md)
- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [Configure Quality of Service for Pods](docs-tasks-configure-pod-container-quality-service-pod.md)
