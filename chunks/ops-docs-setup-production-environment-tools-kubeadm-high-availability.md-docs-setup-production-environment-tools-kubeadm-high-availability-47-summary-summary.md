---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#47-summary
chunk_level: summary
chunk_type: prose
heading: External etcd nodes
token_count: 90
summary: - https://ETCD\_1\_IP:2379 # change ETCD\_1\_IP appropriately - https://ETCD\_2\_IP:2379 # change ETCD\_2\_IP appropriately caFile: /etc/kubernetes/pki/etcd/ca.crt certFile:...
---

- https://ETCD\_1\_IP:2379 # change ETCD\_1\_IP appropriately
- https://ETCD\_2\_IP:2379 # change ETCD\_2\_IP appropriately
caFile: /etc/kubernetes/pki/etcd/ca.crt
certFile: /etc/kubernetes/pki/apiserver-etcd-client.crt
keyFile: /etc/kubernetes/pki/apiserver-etcd-client.key
`
```