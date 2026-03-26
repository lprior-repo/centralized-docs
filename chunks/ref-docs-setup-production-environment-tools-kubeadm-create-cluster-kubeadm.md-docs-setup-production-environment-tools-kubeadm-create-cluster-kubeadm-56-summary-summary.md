---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#56-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 126
summary: Only a few CNI plugins support Windows. More details and setup instructions can be found in [Adding Windows worker nodes](/docs/tasks/administer-cluster/kubeadm/adding-windows-nodes/#network-config)....
---

Only a few CNI plugins support Windows. More details and setup instructions can be found
in [Adding Windows worker nodes](/docs/tasks/administer-cluster/kubeadm/adding-windows-nodes/#network-config).
You can install only one Pod network per cluster.
Once a Pod network has been installed, you can confirm that it is working by
checking that the CoreDNS Pod is `Running` in the output of `kubectl get pods --all-namespaces`.
And once the CoreDNS Pod is up and running, you can continue by joining your nodes.
If your network is not working or CoreDNS is not in the