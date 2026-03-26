---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#43-summary
chunk_level: summary
chunk_type: prose
heading: External etcd nodes
token_count: 111
summary: ``` `export CONTROL\_PLANE=\"ubuntu@10.0.0.7\" scp /etc/kubernetes/pki/etcd/ca.crt \"${CONTROL\_PLANE}\": scp /etc/kubernetes/pki/apiserver-etcd-client.crt \"${CONTROL\_PLANE}\": scp...
---

```
`export CONTROL\_PLANE="ubuntu@10.0.0.7"
scp /etc/kubernetes/pki/etcd/ca.crt "${CONTROL\_PLANE}":
scp /etc/kubernetes/pki/apiserver-etcd-client.crt "${CONTROL\_PLANE}":
scp /etc/kubernetes/pki/apiserver-etcd-client.key "${CONTROL\_PLANE}":
`
```
* Replace the value of `CONTROL\_PLANE` with the `user@host` of the first control-plane node.### Set up the first control plane node