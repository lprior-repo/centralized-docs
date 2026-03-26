---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#11-summary
chunk_level: summary
chunk_type: prose
heading: Overview of Kubernetes Services
token_count: 117
summary: * *ClusterIP* (default) - Exposes the Service on an internal IP in the cluster. This type makes the Service only reachable from within the cluster. * *NodePort* - Exposes the Service on the same port...
---

* *ClusterIP* (default) - Exposes the Service on an internal IP in the cluster. This
type makes the Service only reachable from within the cluster.
* *NodePort* - Exposes the Service on the same port of each selected Node in the cluster using NAT.
Makes a Service accessible from outside the cluster using `NodeIP:NodePort`. Superset of ClusterIP.
* *LoadBalancer* - Creates an external load balancer in the current cloud (if supported)
and assigns a fixed, external IP to the Service. Superset of NodePort.