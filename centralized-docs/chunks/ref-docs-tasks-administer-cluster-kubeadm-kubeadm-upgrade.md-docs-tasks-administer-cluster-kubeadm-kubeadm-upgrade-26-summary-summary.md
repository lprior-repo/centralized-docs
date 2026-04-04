---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#26-summary
chunk_level: summary
chunk_type: prose
heading: Upgrading control plane nodes
token_count: 78
summary: 2. Verify that the download works and has the expected version: ``` `kubeadm version ` ``` 3. Verify the upgrade plan: ``` `sudo kubeadm upgrade plan ` ``` This command checks that your cluster can...
---

2. Verify that the download works and has the expected version:
```
`kubeadm version
`
```
3. Verify the upgrade plan:
```
`sudo kubeadm upgrade plan
`
```
This command checks that your cluster can be upgraded, and fetches the versions you can upgrade to.
It also shows a table with the component config version states.