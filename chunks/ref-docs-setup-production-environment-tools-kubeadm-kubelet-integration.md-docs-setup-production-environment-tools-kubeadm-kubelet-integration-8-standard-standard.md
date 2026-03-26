---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#8-standard
chunk_level: standard
chunk_type: prose
heading: The kubelet drop-in file for systemd
token_count: 298
summary: #### Note: The contents below are just an example. If you don't want to use a package manager follow the guide outlined in the ([Without a package...
---

#### Note:
The contents below are just an example. If you don't want to use a package manager
follow the guide outlined in the ([Without a package manager](/docs/setup/production-environment/tools/kubeadm/install-kubeadm/#k8s-install-2))
section.
```
`[Service]
Environment="KUBELET\_KUBECONFIG\_ARGS=--bootstrap-kubeconfig=/etc/kubernetes/bootstrap-kubelet.conf --kubeconfig=/etc/kubernetes/kubelet.conf"
Environment="KUBELET\_CONFIG\_ARGS=--config=/var/lib/kubelet/config.yaml"
# This is a file that "kubeadm init" and "kubeadm join" generate at runtime, populating
# This is a file that the user can use for overrides of the kubelet args as a last resort. Preferably,
# the user should use the .NodeRegistration.KubeletExtraArgs object in the configuration files instead.
# KUBELET\_EXTRA\_ARGS should be sourced from this file.
EnvironmentFile=-/etc/default/kubelet
ExecStart=
ExecStart=/usr/bin/kubelet $KUBELET\_KUBECONFIG\_ARGS $KUBELET\_CONFIG\_ARGS $KUBELET\_KUBEADM\_ARGS $KUBELET\_EXTRA\_ARGS
`
```
This file specifies the default locations for all of the files managed by kubeadm for the kubelet.