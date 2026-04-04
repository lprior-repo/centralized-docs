---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#4-standard
chunk_level: standard
chunk_type: prose
heading: Core design principles
token_count: 422
summary: * `kubelet.conf` (`bootstrap-kubelet.conf` during TLS bootstrap) * `controller-manager.conf` * `scheduler.conf` * `admin.conf` for the cluster admin and kubeadm itself * `super-admin.conf` for the...
---

* `kubelet.conf` (`bootstrap-kubelet.conf` during TLS bootstrap)
* `controller-manager.conf`
* `scheduler.conf`
* `admin.conf` for the cluster admin and kubeadm itself
* `super-admin.conf` for the cluster super-admin that can bypass RBAC
* Names of certificates and key files:
* `ca.crt`, `ca.key` for the Kubernetes certificate authority
* `apiserver.crt`, `apiserver.key` for the API server certificate
* `apiserver-kubelet-client.crt`, `apiserver-kubelet-client.key` for the client certificate used
by the API server to connect to the kubelets securely
* `sa.pub`, `sa.key` for the key used by the controller manager when signing ServiceAccount
* `front-proxy-ca.crt`, `front-proxy-ca.key` for the front proxy certificate authority
* `front-proxy-client.crt`, `front-proxy-client.key` for the front proxy client## The kubeadm configuration file format
Most kubeadm commands support a `--config` flag which allows passing a configuration file from
disk. The configuration file format follows the common Kubernetes API `apiVersion` / `kind` scheme,
but is considered a component configuration format. Several Kubernetes components, such as the kubelet,
also support file-based configuration.
Different kubeadm subcommands require a different `kind` of configuration file.
For example, `InitConfiguration` for `kubeadm init`, `JoinConfiguration` for `kubeadm join`, `UpgradeConfiguration` for `kubeadm upgrade` and `ResetConfiguration`
for `kubeadm reset`.
The command `kubeadm config migrate` can be used to migrate an older format configuration
file to a newer (current) configuration format. The kubeadm tool only supports migrating from
deprecated configuration formats to the current format.
See the [kubeadm configuration reference](/docs/reference/config-api/kubeadm-config.v1beta4/) page for more details.