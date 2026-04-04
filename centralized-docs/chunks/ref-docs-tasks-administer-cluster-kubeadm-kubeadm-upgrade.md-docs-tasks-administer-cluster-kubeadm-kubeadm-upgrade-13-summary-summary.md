---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#13-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 110
summary: * All containers are restarted after upgrade, because the container spec hash value is changed. * To verify that the kubelet service has successfully restarted after the kubelet has been upgraded,...
---

* All containers are restarted after upgrade, because the container spec hash value is changed.
* To verify that the kubelet service has successfully restarted after the kubelet has been upgraded,
you can execute `systemctl status kubelet` or view the service logs with `journalctl -xeu kubelet`.
* `kubeadm upgrade` supports `--config` with a
[`UpgradeConfiguration` API type](/docs/reference/config-api/kubeadm-config.v1beta4/) which can
be used to configure the upgrade process.