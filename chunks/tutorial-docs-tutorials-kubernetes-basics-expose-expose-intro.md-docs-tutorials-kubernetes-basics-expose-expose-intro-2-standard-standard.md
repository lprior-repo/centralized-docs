---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#2-standard
chunk_level: standard
chunk_type: prose
heading: Overview of Kubernetes Services
token_count: 321
summary: * *ClusterIP* (default) - Exposes the Service on an internal IP in the cluster. This type makes the Service only reachable from within the cluster. * *NodePort* - Exposes the Service on the same port...
---

* *ClusterIP* (default) - Exposes the Service on an internal IP in the cluster. This
type makes the Service only reachable from within the cluster.
* *NodePort* - Exposes the Service on the same port of each selected Node in the cluster using NAT.
Makes a Service accessible from outside the cluster using `NodeIP:NodePort`. Superset of ClusterIP.
* *LoadBalancer* - Creates an external load balancer in the current cloud (if supported)
and assigns a fixed, external IP to the Service. Superset of NodePort.
* *ExternalName* - Maps the Service to the contents of the `externalName` field
(e.g. `foo.bar.example.com`), by returning a `CNAME` record with its value.
No proxying of any kind is set up. This type requires v1.7 or higher of `kube-dns`,
or CoreDNS version 0.0.8 or higher.
More information about the different types of Services can be found in the
[Using Source IP](/docs/tutorials/services/source-ip/) tutorial. Also see
[Connecting Applications with Services](/docs/tutorials/services/connect-applications-service/).
Additionally, note that there are some use cases with Services that involve not defining
a `selector` in the spec. A Service created without `selector` will also not create
the corresponding Endpoints object. This allows users to manually map a Service to
specific endpoints. Another possibility why there may be no selector is you are strictly
using `type: ExternalName`.