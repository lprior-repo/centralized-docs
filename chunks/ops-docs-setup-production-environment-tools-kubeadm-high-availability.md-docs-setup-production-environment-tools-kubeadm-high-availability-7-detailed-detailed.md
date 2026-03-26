---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#7-detailed
chunk_level: detailed
chunk_type: code
heading: Feedback
token_count: 1016
summary: ## Manual certificate distribution If you choose to not use `kubeadm init` with the `--upload-certs` flag this means that you are going to have to manually copy the certificates from the primary...
---

## Manual certificate distribution
If you choose to not use `kubeadm init` with the `--upload-certs` flag this means that
you are going to have to manually copy the certificates from the primary control plane node to the
joining control plane nodes.
There are many ways to do this. The following example uses `ssh` and `scp`:
SSH is required if you want to control all nodes from a single machine.
1. Enable ssh-agent on your main device that has access to all other nodes in
the system:
```
`eval $(ssh-agent)
`
```
2. Add your SSH identity to the session:
```
`ssh-add \~/.ssh/path\_to\_private\_key
`
```
3. SSH between nodes to check that the connection is working correctly.
* When you SSH to any node, add the `-A` flag. This flag allows the node that you
have logged into via SSH to access the SSH agent on your PC. Consider alternative
methods if you do not fully trust the security of your user session on the node.
```
`ssh -A 10.0.0.7
`
```
* When using sudo on any node, make sure to preserve the environment so SSH
forwarding works:
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
#### Caution:
Copy only the certificates in the above list. kubeadm will take care of generating the rest of the certificates
with the required SANs for the joining control-plane instances. If you copy all the certificates by mistake,
the creation of additional nodes could fail due to a lack of required SANs.
* Then on each joining control plane node you have to run the following script before running `kubeadm join`.
This script will move the previously copied certificates from the home directory to `/etc/kubernetes/pki`:
```
`USER=ubuntu # customizable
mkdir -p /etc/kubernetes/pki/etcd
mv /home/${USER}/ca.crt /etc/kubernetes/pki/
mv /home/${USER}/ca.key /etc/kubernetes/pki/
mv /home/${USER}/sa.pub /etc/kubernetes/pki/
mv /home/${USER}/sa.key /etc/kubernetes/pki/
mv /home/${USER}/front-proxy-ca.crt /etc/kubernetes/pki/
mv /home/${USER}/front-proxy-ca.key /etc/kubernetes/pki/
mv /home/${USER}/etcd-ca.crt /etc/kubernetes/pki/etcd/ca.crt
# Skip the next line if you are using external etcd
mv /home/${USER}/etcd-ca.key /etc/kubernetes/pki/etcd/ca.key
`
```
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified August 31, 2025 at 6:55 PM PST: [Remove parallel join note (abf3d4dfb3)](https://github.com/kubernetes/website/commit/abf3d4dfb368e77d44fbf3908eb3214b99959593)