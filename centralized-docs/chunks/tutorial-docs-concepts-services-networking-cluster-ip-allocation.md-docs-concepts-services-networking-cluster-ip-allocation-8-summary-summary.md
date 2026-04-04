---
doc_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation
chunk_id: tutorial/docs-concepts-services-networking-cluster-ip-allocation.md/docs-concepts-services-networking-cluster-ip-allocation#8-summary
chunk_level: summary
chunk_type: prose
heading: Why do you need to reserve Service Cluster IPs?
token_count: 128
summary: `apiVersion: v1 kind: Service metadata: labels: k8s-app: kube-dns kubernetes.io/cluster-service: \"true\" kubernetes.io/name: CoreDNS name: kube-dns namespace: kube-system spec: clusterIP: 10.96.0.10...
---

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