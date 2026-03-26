---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#62-summary
chunk_level: summary
chunk_type: prose
heading: Manual certificate distribution
token_count: 127
summary: `USER=ubuntu # customizable mkdir -p /etc/kubernetes/pki/etcd mv /home/${USER}/ca.crt /etc/kubernetes/pki/ mv /home/${USER}/ca.key /etc/kubernetes/pki/ mv /home/${USER}/sa.pub /etc/kubernetes/pki/ mv...
---

`USER=ubuntu # customizable
mkdir -p /etc/kubernetes/pki/etcd
mv /home/${USER}/ca.crt /etc/kubernetes/pki/
mv /home/${USER}/ca.key /etc/kubernetes/pki/
mv /home/${USER}/sa.pub /etc/kubernetes/pki/
mv /home/${USER}/sa.key /etc/kubernetes/pki/
mv /home/${USER}/front-proxy-ca.crt /etc/kubernetes/pki/
mv /home/${USER}/front-proxy-ca.key /etc/kubernetes/pki/
mv /home/${USER}/etcd-ca.crt /etc/kubernetes/pki/etcd/ca.crt