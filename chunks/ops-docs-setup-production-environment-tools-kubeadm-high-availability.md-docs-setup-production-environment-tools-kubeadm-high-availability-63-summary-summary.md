---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#63-summary
chunk_level: summary
chunk_type: prose
heading: Manual certificate distribution
token_count: 57
summary: mv /home/${USER}/etcd-ca.crt /etc/kubernetes/pki/etcd/ca.crt # Skip the next line if you are using external etcd mv /home/${USER}/etcd-ca.key /etc/kubernetes/pki/etcd/ca.key ` ```
---

mv /home/${USER}/etcd-ca.crt /etc/kubernetes/pki/etcd/ca.crt
# Skip the next line if you are using external etcd
mv /home/${USER}/etcd-ca.key /etc/kubernetes/pki/etcd/ca.key
`
```