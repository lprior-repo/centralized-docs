---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#38-summary
chunk_level: summary
chunk_type: prose
heading: The kubelet drop-in file for systemd
token_count: 125
summary: Preferably, # the user should use the .NodeRegistration.KubeletExtraArgs object in the configuration files instead. # KUBELET\_EXTRA\_ARGS should be sourced from this file....
---

Preferably,
# the user should use the .NodeRegistration.KubeletExtraArgs object in the configuration files instead.
# KUBELET\_EXTRA\_ARGS should be sourced from this file.
EnvironmentFile=-/etc/default/kubelet
ExecStart=
ExecStart=/usr/bin/kubelet $KUBELET\_KUBECONFIG\_ARGS $KUBELET\_CONFIG\_ARGS $KUBELET\_KUBEADM\_ARGS $KUBELET\_EXTRA\_ARGS
`
```
This file specifies the default locations for all of the files managed by kubeadm for the kubelet.