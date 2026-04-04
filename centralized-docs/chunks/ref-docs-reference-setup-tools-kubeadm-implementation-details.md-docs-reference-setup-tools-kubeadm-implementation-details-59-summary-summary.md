---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#59-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 126
summary: * `--requestheader-username-headers=X-Remote-User` * `--requestheader-group-headers=X-Remote-Group` * `--requestheader-extra-headers-prefix=X-Remote-Extra-` *...
---

* `--requestheader-username-headers=X-Remote-User`
* `--requestheader-group-headers=X-Remote-Group`
* `--requestheader-extra-headers-prefix=X-Remote-Extra-`
* `--requestheader-allowed-names=front-proxy-client`#### Controller manager
The static Pod manifest for the controller manager is affected by following parameters provided by
the users:
* If kubeadm is invoked specifying a `--pod-network-cidr`, the subnet manager feature required for
some CNI network plugins is enabled by setting:
* `--allocate-node-cidrs=true`