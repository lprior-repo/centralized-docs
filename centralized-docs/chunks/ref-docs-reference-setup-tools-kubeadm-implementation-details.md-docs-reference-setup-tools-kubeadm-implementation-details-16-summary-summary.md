---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#16-summary
chunk_level: summary
chunk_type: prose
heading: Core design principles
token_count: 126
summary: * `front-proxy-client.crt`, `front-proxy-client.key` for the front proxy client## The kubeadm configuration file format Most kubeadm commands support a `--config` flag which allows passing a...
---

* `front-proxy-client.crt`, `front-proxy-client.key` for the front proxy client## The kubeadm configuration file format
Most kubeadm commands support a `--config` flag which allows passing a configuration file from
disk. The configuration file format follows the common Kubernetes API `apiVersion` / `kind` scheme,
but is considered a component configuration format. Several Kubernetes components, such as the kubelet,
also support file-based configuration.
Different kubeadm subcommands require a different `kind` of configuration file.
For example, `InitConfiguration` for `kubeadm init`,