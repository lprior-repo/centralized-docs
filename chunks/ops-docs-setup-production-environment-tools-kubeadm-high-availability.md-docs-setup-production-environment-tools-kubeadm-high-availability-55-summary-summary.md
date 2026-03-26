---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#55-summary
chunk_level: summary
chunk_type: prose
heading: Manual certificate distribution
token_count: 112
summary: ``` `eval $(ssh-agent) ` ``` 2. Add your SSH identity to the session: ``` `ssh-add \~/.ssh/path\_to\_private\_key ` ``` 3. SSH between nodes to check that the connection is working correctly. * When...
---

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