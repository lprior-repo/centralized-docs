---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#56-summary
chunk_level: summary
chunk_type: prose
heading: Manual certificate distribution
token_count: 127
summary: ``` `ssh -A 10.0.0.7 ` ``` * When using sudo on any node, make sure to preserve the environment so SSH forwarding works: ``` `sudo -E -s ` ``` * After configuring SSH on all the nodes you should run...
---

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