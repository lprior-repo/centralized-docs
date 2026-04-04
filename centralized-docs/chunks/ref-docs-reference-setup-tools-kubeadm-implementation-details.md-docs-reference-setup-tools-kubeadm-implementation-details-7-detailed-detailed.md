---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#7-detailed
chunk_level: detailed
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 970
summary: * Other flags for securing the front proxy ([API Aggregation](/docs/concepts/extend-kubernetes/api-extension/apiserver-aggregation/)) communications: *...
---

* Other flags for securing the front proxy
([API Aggregation](/docs/concepts/extend-kubernetes/api-extension/apiserver-aggregation/))
communications:
* `--requestheader-username-headers=X-Remote-User`
* `--requestheader-group-headers=X-Remote-Group`
* `--requestheader-extra-headers-prefix=X-Remote-Extra-`
* `--requestheader-allowed-names=front-proxy-client`#### Controller manager
The static Pod manifest for the controller manager is affected by following parameters provided by
the users:
* If kubeadm is invoked specifying a `--pod-network-cidr`, the subnet manager feature required for
some CNI network plugins is enabled by setting:
* `--allocate-node-cidrs=true`
* `--cluster-cidr` and `--node-cidr-mask-size` flags according to the given CIDR
Other flags that are set unconditionally are:
* `--controllers` enabling all the default controllers plus `BootstrapSigner` and `TokenCleaner`
controllers for TLS bootstrap. See [TLS Bootstrapping](/docs/reference/access-authn-authz/kubelet-tls-bootstrapping/)
for more details.
* `--use-service-account-credentials` to `true`
* Flags for using certificates generated in previous steps:
* `--root-ca-file` to `ca.crt`
* `--cluster-signing-cert-file` to `ca.crt`, if External CA mode is disabled, otherwise to `""`
* `--cluster-signing-key-file` to `ca.key`, if External CA mode is disabled, otherwise to `""`
* `--service-account-private-key-file` to `sa.key`#### Scheduler
The static Pod manifest for the scheduler is not affected by parameters provided by the user.
### Generate static Pod manifest for local etcd
If you specified an external etcd, this step will be skipped, otherwise kubeadm generates a
static Pod manifest file for creating a local etcd instance running in a Pod with following attributes:
* listen on `localhost:2379` and use `HostNetwork=true`
* make a `hostPath` mount out from the `dataDir` to the host's filesystem
* Any extra flags specified by the user
Please note that:
1. The etcd container image will be pulled from `registry.gcr.io` by default. See
[using custom images](/docs/reference/setup-tools/kubeadm/kubeadm-init/#custom-images)
for customizing the image repository.
2. If you run kubeadm in `--dry-run` mode, the etcd static Pod manifest is written
into a temporary folder.
3. You can directly invoke static Pod manifest generation for local etcd, using the
[`kubeadm init phase etcd local`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-etcd)
command.### Wait for the control plane to come up
On control plane nodes, kubeadm waits up to 4 minutes for the control plane components
and the kubelet to be available. It does that by performing a health check on the respective
component `/healthz` or `/livez` endpoints.
After the control plane is up, kubeadm completes the tasks described in following paragraphs.
### Save the kubeadm ClusterConfiguration in a ConfigMap for later reference
kubeadm saves the configuration passed to `kubeadm init` in a ConfigMap named `kubeadm-config`
under `kube-system` namespace.
This will ensure that kubeadm actions executed in future (e.g `kubeadm upgrade`) will be able to
determine the actual/current cluster state and make new decisions based on that data.
Please note that:
1. Before saving the ClusterConfiguration, sensitive information like the token is stripped from the configuration
2. Upload of control plane node configuration can be invoked individually with the command
[`kubeadm init phase upload-config`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-upload-config).### Mark the node as control-plane
As soon as the control plane is available, kubeadm executes the following actions:
* Labels the node as control-plane with `node-role.kubernetes.io/control-plane=""`
* Taints the node with `node-role.kubernetes.io/control-plane:NoSchedule`
Please note that the phase to mark the control-plane phase can be invoked
individually with the [`kubeadm init phase mark-control-plane`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-mark-control-plane) command.