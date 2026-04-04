---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#16-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 56
summary: ``` `killall -s SIGTERM kube-apiserver # trigger a graceful kube-apiserver shutdown sleep 20 # wait a little bit to permit completing in-flight requests kubeadm upgrade ... # execute a kubeadm...
---

```
`killall -s SIGTERM kube-apiserver # trigger a graceful kube-apiserver shutdown
sleep 20 # wait a little bit to permit completing in-flight requests
kubeadm upgrade ... # execute a kubeadm upgrade command
`
```