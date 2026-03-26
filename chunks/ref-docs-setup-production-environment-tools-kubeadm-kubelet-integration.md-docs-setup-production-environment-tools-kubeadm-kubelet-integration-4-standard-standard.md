---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#4-standard
chunk_level: standard
chunk_type: prose
heading: Configure kubelets using kubeadm
token_count: 162
summary: ## Configure kubelets using kubeadm It is possible to configure the kubelet that kubeadm will start if a custom [`KubeletConfiguration`](/docs/reference/config-api/kubelet-config.v1beta1/) API object...
---

## Configure kubelets using kubeadm
It is possible to configure the kubelet that kubeadm will start if a custom
[`KubeletConfiguration`](/docs/reference/config-api/kubelet-config.v1beta1/)
API object is passed with a configuration file like so `kubeadm ... --config some-config-file.yaml`.
By calling `kubeadm config print init-defaults --component-configs KubeletConfiguration` you can
see all the default values for this structure.
It is also possible to apply instance-specific patches over the base `KubeletConfiguration`.
Have a look at [Customizing the kubelet](/docs/setup/production-environment/tools/kubeadm/control-plane-flags/#customizing-the-kubelet)
for more details.