---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#14-standard
chunk_level: standard
chunk_type: prose
heading: Manual certificate distribution
token_count: 284
summary: ``` `sudo -E -s ` ``` * After configuring SSH on all the nodes you should run the following script on the first control plane node after running `kubeadm init`. This script will copy the certificates...
---

```
`sudo -E -s
`
```
* After configuring SSH on all the nodes you should run the following script on the first
control plane node after running `kubeadm init`. This script will copy the certificates from
the first control plane node to the other control plane nodes:
In the following example, replace `CONTROL\_PLANE\_IPS` with the IP addresses of the
other control plane nodes.
```
`USER=ubuntu # customizable
CONTROL\_PLANE\_IPS="10.0.0.7 10.0.0.8"
for host in ${CONTROL\_PLANE\_IPS}; do
scp /etc/kubernetes/pki/ca.crt "${USER}"@$host:
scp /etc/kubernetes/pki/ca.key "${USER}"@$host:
scp /etc/kubernetes/pki/sa.key "${USER}"@$host:
scp /etc/kubernetes/pki/sa.pub "${USER}"@$host:
scp /etc/kubernetes/pki/front-proxy-ca.crt "${USER}"@$host:
scp /etc/kubernetes/pki/front-proxy-ca.key "${USER}"@$host:
scp /etc/kubernetes/pki/etcd/ca.crt "${USER}"@$host:etcd-ca.crt
# Skip the next line if you are using external etcd
scp /etc/kubernetes/pki/etcd/ca.key "${USER}"@$host:etcd-ca.key
done
`
```