---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#62-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 101
summary: ### Generate static Pod manifest for local etcd If you specified an external etcd, this step will be skipped, otherwise kubeadm generates a static Pod manifest file for creating a local etcd instance...
---

### Generate static Pod manifest for local etcd
If you specified an external etcd, this step will be skipped, otherwise kubeadm generates a
static Pod manifest file for creating a local etcd instance running in a Pod with following attributes:
* listen on `localhost:2379` and use `HostNetwork=true`
* make a `hostPath` mount out from the `dataDir` to the host's filesystem
* Any extra flags specified by the user
Please note that: