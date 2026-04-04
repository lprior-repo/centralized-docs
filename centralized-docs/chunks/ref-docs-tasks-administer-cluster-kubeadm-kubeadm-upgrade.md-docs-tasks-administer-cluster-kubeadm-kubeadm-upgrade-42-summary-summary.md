---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#42-summary
chunk_level: summary
chunk_type: prose
heading: Recovering from a failure state
token_count: 125
summary: * `kubeadm-backup-manifests-&lt;date&gt;-&lt;time&gt;` `kubeadm-backup-etcd` contains a backup of the local etcd member data for this control plane Node. In case of an etcd upgrade failure and if the...
---

* `kubeadm-backup-manifests-&lt;date&gt;-&lt;time&gt;`
`kubeadm-backup-etcd` contains a backup of the local etcd member data for this control plane Node.
In case of an etcd upgrade failure and if the automatic rollback does not work, the contents of this folder
can be manually restored in `/var/lib/etcd`. In case external etcd is used this backup folder will be empty.
`kubeadm-backup-manifests` contains a backup of the static Pod manifest files for this control plane Node.