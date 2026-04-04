---
doc_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation
chunk_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Why do you need to reserve Service Cluster IPs?
token_count: 530
summary: # Service ClusterIP allocation In Kubernetes, [Services](/docs/concepts/services-networking/service/) are an abstract way to expose an application running on a set of Pods. Services can have a...
---

# Service ClusterIP allocation
In Kubernetes, [Services](/docs/concepts/services-networking/service/) are an abstract way to expose
an application running on a set of Pods. Services
can have a cluster-scoped virtual IP address (using a Service of `type: ClusterIP`).
Clients can connect using that virtual IP address, and Kubernetes then load-balances traffic to that
Service across the different backing Pods.
## How Service ClusterIPs are allocated?
When Kubernetes needs to assign a virtual IP address for a Service,
that assignment happens one of two ways:
*dynamically*the cluster's control plane automatically picks a free IP address from within the configured IP range for `type: ClusterIP` Services.*statically*you specify an IP address of your choice, from within the configured IP range for Services.
Across your whole cluster, every Service `ClusterIP` must be unique.
Trying to create a Service with a specific `ClusterIP` that has already
been allocated will return an error.
## Why do you need to reserve Service Cluster IPs?
Sometimes you may want to have Services running in well-known IP addresses, so other components and
users in the cluster can use them.
The best example is the DNS Service for the cluster. As a soft convention, some Kubernetes installers assign the 10th IP address from
the Service IP range to the DNS service. Assuming you configured your cluster with Service IP range
10.96.0.0/16 and you want your DNS Service IP to be 10.96.0.10, you'd have to create a Service like
this:
```
`apiVersion: v1
kind: Service
metadata:
labels:
k8s-app: kube-dns
kubernetes.io/cluster-service: "true"
kubernetes.io/name: CoreDNS
name: kube-dns
namespace: kube-system
spec:
clusterIP: 10.96.0.10
ports:
- name: dns
port: 53
protocol: UDP
targetPort: 53
- name: dns-tcp
port: 53
protocol: TCP
targetPort: 53
selector:
k8s-app: kube-dns
type: ClusterIP
`
```
But, as it was explained before, the IP address 10.96.0.10 has not been reserved.
If other Services are created before or in parallel with dynamic allocation, there is a chance they can allocate this IP.
Hence, you will not be able to create the DNS Service because it will fail with a conflict error.