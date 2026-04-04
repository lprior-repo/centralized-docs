---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#15-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 499
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