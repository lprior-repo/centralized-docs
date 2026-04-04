---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#27-summary
chunk_level: summary
chunk_type: prose
heading: Upgrading control plane nodes
token_count: 120
summary: #### Note: `kubeadm upgrade` also automatically renews the certificates that it manages on this node. To opt-out of certificate renewal the flag `--certificate-renewal=false` can be used. For more...
---

#### Note:
`kubeadm upgrade` also automatically renews the certificates that it manages on this node.
To opt-out of certificate renewal the flag `--certificate-renewal=false` can be used.
For more information see the [certificate management guide](/docs/tasks/administer-cluster/kubeadm/kubeadm-certs/).
4. Choose a version to upgrade to, and run the appropriate command. For example:
```
`# replace x with the patch version you picked for this upgrade
sudo kubeadm upgrade apply v1.35.x
`
```