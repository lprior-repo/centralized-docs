---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#3-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 342
summary: * To verify that the kubelet service has successfully restarted after the kubelet has been upgraded, you can execute `systemctl status kubelet` or view the service logs with `journalctl -xeu...
---

* To verify that the kubelet service has successfully restarted after the kubelet has been upgraded,
you can execute `systemctl status kubelet` or view the service logs with `journalctl -xeu kubelet`.
* `kubeadm upgrade` supports `--config` with a
[`UpgradeConfiguration` API type](/docs/reference/config-api/kubeadm-config.v1beta4/) which can
be used to configure the upgrade process.
* `kubeadm upgrade` does not support reconfiguration of an existing cluster. Follow the steps in
[Reconfiguring a kubeadm cluster](/docs/tasks/administer-cluster/kubeadm/kubeadm-reconfigure/) instead.### Considerations when upgrading etcd
Because the `kube-apiserver` static pod is running at all times (even if you
have drained the node), when you perform a kubeadm upgrade which includes an
etcd upgrade, in-flight requests to the server will stall while the new etcd
static pod is restarting. As a workaround, it is possible to actively stop the
`kube-apiserver` process a few seconds before starting the `kubeadm upgrade apply` command. This permits to complete in-flight requests and close existing
connections, and minimizes the consequence of the etcd downtime. This can be
done as follows on control plane nodes:
```
`killall -s SIGTERM kube-apiserver # trigger a graceful kube-apiserver shutdown
sleep 20 # wait a little bit to permit completing in-flight requests
kubeadm upgrade ... # execute a kubeadm upgrade command
`
```