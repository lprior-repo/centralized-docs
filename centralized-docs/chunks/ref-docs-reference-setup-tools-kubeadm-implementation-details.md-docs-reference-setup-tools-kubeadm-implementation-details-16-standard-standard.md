---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#16-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 506
summary: * [`PersistentVolumeLabel`](/docs/reference/access-authn-authz/admission-controllers/#persistentvolumelabel) attaches region or zone labels to PersistentVolumes as defined by the cloud provider (This...
---

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