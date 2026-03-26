---
doc_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller
chunk_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller#9-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 97
summary: * cloud authentication/authorization: your cloud may require a token or IAM rules to allow access to their APIs * kubernetes authentication/authorization: cloud-controller-manager may need RBAC rules...
---

* cloud authentication/authorization: your cloud may require a token or IAM rules
to allow access to their APIs
* kubernetes authentication/authorization: cloud-controller-manager may need RBAC
rules set to speak to the kubernetes apiserver
* high availability: like kube-controller-manager, you may want a high available
setup for cloud controller manager using leader election (on by default).### Running cloud-controller-manager
Successfully running cloud-controller-manager requires some changes to your cluster configuration.