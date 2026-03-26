---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#59-summary
chunk_level: summary
chunk_type: prose
heading: Manual certificate distribution
token_count: 93
summary: scp /etc/kubernetes/pki/front-proxy-ca.crt \"${USER}\"@$host: scp /etc/kubernetes/pki/front-proxy-ca.key \"${USER}\"@$host: scp /etc/kubernetes/pki/etcd/ca.crt \"${USER}\"@$host:etcd-ca.crt # Skip the next...
---

scp /etc/kubernetes/pki/front-proxy-ca.crt "${USER}"@$host:
scp /etc/kubernetes/pki/front-proxy-ca.key "${USER}"@$host:
scp /etc/kubernetes/pki/etcd/ca.crt "${USER}"@$host:etcd-ca.crt
# Skip the next line if you are using external etcd
scp /etc/kubernetes/pki/etcd/ca.key "${USER}"@$host:etcd-ca.key
done
`
```