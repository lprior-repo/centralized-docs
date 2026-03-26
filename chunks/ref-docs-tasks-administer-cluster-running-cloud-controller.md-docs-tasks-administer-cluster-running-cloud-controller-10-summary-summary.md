---
doc_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller
chunk_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller#10-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 92
summary: * `kubelet` and `kube-controller-manager` must be set according to the user's usage of external CCM. If the user has an external CCM (not the internal cloud controller loops in the Kubernetes...
---

* `kubelet` and `kube-controller-manager` must be set according to the
user's usage of external CCM. If the user has an external CCM (not the internal cloud
controller loops in the Kubernetes Controller Manager), then `--cloud-provider=external`
must be specified. Otherwise, it should not be specified.
Keep in mind that setting up your cluster to use cloud controller manager will
change your cluster behaviour in a few ways: