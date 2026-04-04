---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#10-standard
chunk_level: standard
chunk_type: prose
heading: Recovering from a failure state
token_count: 368
summary: ## Recovering from a failure state If `kubeadm upgrade` fails and does not roll back, for example because of an unexpected shutdown during execution, you can run `kubeadm upgrade` again. This command...
---

## Recovering from a failure state
If `kubeadm upgrade` fails and does not roll back, for example because of an unexpected shutdown during execution, you can run `kubeadm upgrade` again.
This command is idempotent and eventually makes sure that the actual state is the desired state you declare.
To recover from a bad state, you can also run `sudo kubeadm upgrade apply --force` without changing the version that your cluster is running.
During upgrade kubeadm writes the following backup folders under `/etc/kubernetes/tmp`:
* `kubeadm-backup-etcd-&lt;date&gt;-&lt;time&gt;`
* `kubeadm-backup-manifests-&lt;date&gt;-&lt;time&gt;`
`kubeadm-backup-etcd` contains a backup of the local etcd member data for this control plane Node.
In case of an etcd upgrade failure and if the automatic rollback does not work, the contents of this folder
can be manually restored in `/var/lib/etcd`. In case external etcd is used this backup folder will be empty.
`kubeadm-backup-manifests` contains a backup of the static Pod manifest files for this control plane Node.
In case of a upgrade failure and if the automatic rollback does not work, the contents of this folder can be
manually restored in `/etc/kubernetes/manifests`. If for some reason there is no difference between a pre-upgrade
and post-upgrade manifest file for a certain component, a backup file for it will not be written.
#### Note:
After the cluster upgrade using kubeadm, the backup directory `/etc/kubernetes/tmp` will remain and
these backup files will need to be cleared manually.