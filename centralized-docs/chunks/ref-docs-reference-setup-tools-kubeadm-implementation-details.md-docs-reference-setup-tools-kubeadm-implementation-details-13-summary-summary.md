---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#13-summary
chunk_level: summary
chunk_type: prose
heading: Core design principles
token_count: 115
summary: * `/etc/kubernetes/manifests` as the path where the kubelet should look for static Pod manifests. Names of static Pod manifests are: * `etcd.yaml` * `kube-apiserver.yaml` *...
---

* `/etc/kubernetes/manifests` as the path where the kubelet should look for static Pod manifests.
Names of static Pod manifests are:
* `etcd.yaml`
* `kube-apiserver.yaml`
* `kube-controller-manager.yaml`
* `kube-scheduler.yaml`
* `/etc/kubernetes/` as the path where kubeconfig files with identities for control plane
components are stored. Names of kubeconfig files are:
* `kubelet.conf` (`bootstrap-kubelet.conf` during TLS bootstrap)
* `controller-manager.conf`
* `scheduler.conf`