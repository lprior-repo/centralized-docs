---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#12-summary
chunk_level: summary
chunk_type: prose
heading: Core design principles
token_count: 107
summary: * It should provide the possibility to use a config file for customizing various parameters## Constants and well-known values and paths In order to reduce complexity and to simplify development of...
---

* It should provide the possibility to use a config file for customizing various parameters## Constants and well-known values and paths
In order to reduce complexity and to simplify development of higher level tools that build on top of kubeadm, it uses a
limited set of constant values for well-known paths and file names.
The Kubernetes directory `/etc/kubernetes` is a constant in the application, since it is clearly the given path
in a majority of cases, and the most intuitive location; other constant paths and file names are: