---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#46-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 106
summary: * All static Pods are deployed on `kube-system` namespace * All static Pods get `tier:control-plane` and `component:{component-name}` labels * All static Pods use the `system-node-critical` priority...
---

* All static Pods are deployed on `kube-system` namespace
* All static Pods get `tier:control-plane` and `component:{component-name}` labels
* All static Pods use the `system-node-critical` priority class
* `hostNetwork: true` is set on all static Pods to allow control plane startup before a network is
configured; as a consequence:
* The `address` that the controller-manager and the scheduler use to refer to the API server is `127.0.0.1`