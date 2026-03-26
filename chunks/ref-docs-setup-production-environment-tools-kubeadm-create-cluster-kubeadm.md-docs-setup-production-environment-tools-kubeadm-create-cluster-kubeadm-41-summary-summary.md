---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#41-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 78
summary: ``` `mkdir -p $HOME/.kube sudo cp -i /etc/kubernetes/admin.conf $HOME/.kube/config sudo chown $(id -u):$(id -g) $HOME/.kube/config ` ``` Alternatively, if you are the `root` user, you can run: ```...
---

```
`mkdir -p $HOME/.kube
sudo cp -i /etc/kubernetes/admin.conf $HOME/.kube/config
sudo chown $(id -u):$(id -g) $HOME/.kube/config
`
```
Alternatively, if you are the `root` user, you can run:
```
`export KUBECONFIG=/etc/kubernetes/admin.conf
`
```