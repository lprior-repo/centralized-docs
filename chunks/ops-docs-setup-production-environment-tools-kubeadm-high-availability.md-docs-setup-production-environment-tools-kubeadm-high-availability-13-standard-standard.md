---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#13-standard
chunk_level: standard
chunk_type: prose
heading: Manual certificate distribution
token_count: 350
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