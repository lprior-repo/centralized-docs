---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#58-summary
chunk_level: summary
chunk_type: prose
heading: Manual certificate distribution
token_count: 117
summary: `USER=ubuntu # customizable CONTROL\_PLANE\_IPS=\"10.0.0.7 10.0.0.8\" for host in ${CONTROL\_PLANE\_IPS}; do scp /etc/kubernetes/pki/ca.crt \"${USER}\"@$host: scp /etc/kubernetes/pki/ca.key...
---

`USER=ubuntu # customizable
CONTROL\_PLANE\_IPS="10.0.0.7 10.0.0.8"
for host in ${CONTROL\_PLANE\_IPS}; do
scp /etc/kubernetes/pki/ca.crt "${USER}"@$host:
scp /etc/kubernetes/pki/ca.key "${USER}"@$host:
scp /etc/kubernetes/pki/sa.key "${USER}"@$host:
scp /etc/kubernetes/pki/sa.pub "${USER}"@$host:
scp /etc/kubernetes/pki/front-proxy-ca.crt "${USER}"@$host: