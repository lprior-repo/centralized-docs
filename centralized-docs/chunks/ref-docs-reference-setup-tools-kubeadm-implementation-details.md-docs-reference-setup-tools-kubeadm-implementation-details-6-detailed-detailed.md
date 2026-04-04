---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#6-detailed
chunk_level: detailed
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 992
summary: * The `apiserver-advertise-address` and `apiserver-bind-port` to bind to; if not provided, those values default to the IP address of the default network interface on the machine and port 6443 * The...
---

* The `apiserver-advertise-address` and `apiserver-bind-port` to bind to; if not provided, those
values default to the IP address of the default network interface on the machine and port 6443
* The `service-cluster-ip-range` to use for services
* If an external etcd server is specified, the `etcd-servers` address and related TLS settings
(`etcd-cafile`, `etcd-certfile`, `etcd-keyfile`);
if an external etcd server is not provided, a local etcd will be used (via host network)
* If a cloud provider is specified, the corresponding `--cloud-provider` parameter is configured together
with the `--cloud-config` path if such file exists (this is experimental, alpha and will be
removed in a future version)
Other API server flags that are set unconditionally are:
* `--insecure-port=0` to avoid insecure connections to the api server
* `--enable-bootstrap-token-auth=true` to enable the `BootstrapTokenAuthenticator` authentication module.
See [TLS Bootstrapping](/docs/reference/access-authn-authz/kubelet-tls-bootstrapping/) for more details
* `--allow-privileged` to `true` (required e.g. by kube proxy)
* `--requestheader-client-ca-file` to `front-proxy-ca.crt`
* `--enable-admission-plugins` to:
* [`NamespaceLifecycle`](/docs/reference/access-authn-authz/admission-controllers/#namespacelifecycle)
e.g. to avoid deletion of system reserved namespaces
* [`LimitRanger`](/docs/reference/access-authn-authz/admission-controllers/#limitranger)
and [`ResourceQuota`](/docs/reference/access-authn-authz/admission-controllers/#resourcequota)
to enforce limits on namespaces
* [`ServiceAccount`](/docs/reference/access-authn-authz/admission-controllers/#serviceaccount)
to enforce service account automation
* [`PersistentVolumeLabel`](/docs/reference/access-authn-authz/admission-controllers/#persistentvolumelabel)
attaches region or zone labels to PersistentVolumes as defined by the cloud provider (This
admission controller is deprecated and will be removed in a future version.
It is not deployed by kubeadm by default with v1.9 onwards when not explicitly opting into
using `gce` or `aws` as cloud providers)
* [`DefaultStorageClass`](/docs/reference/access-authn-authz/admission-controllers/#defaultstorageclass)
to enforce default storage class on `PersistentVolumeClaim` objects
* [`DefaultTolerationSeconds`](/docs/reference/access-authn-authz/admission-controllers/#defaulttolerationseconds)
* [`NodeRestriction`](/docs/reference/access-authn-authz/admission-controllers/#noderestriction)
to limit what a kubelet can modify (e.g. only pods on this node)
* `--kubelet-preferred-address-types` to `InternalIP,ExternalIP,Hostname;` this makes `kubectl logs` and other API server-kubelet communication work in environments where the hostnames of the
nodes aren't resolvable
* Flags for using certificates generated in previous steps:
* `--client-ca-file` to `ca.crt`
* `--tls-cert-file` to `apiserver.crt`
* `--tls-private-key-file` to `apiserver.key`
* `--kubelet-client-certificate` to `apiserver-kubelet-client.crt`
* `--kubelet-client-key` to `apiserver-kubelet-client.key`
* `--service-account-key-file` to `sa.pub`
* `--requestheader-client-ca-file` to `front-proxy-ca.crt`
* `--proxy-client-cert-file` to `front-proxy-client.crt`
* `--proxy-client-key-file` to `front-proxy-client.key`
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