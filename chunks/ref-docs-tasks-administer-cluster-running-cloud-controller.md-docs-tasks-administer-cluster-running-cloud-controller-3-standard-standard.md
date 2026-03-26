---
doc_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller
chunk_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller#3-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 469
summary: * cloud authentication/authorization: your cloud may require a token or IAM rules to allow access to their APIs * kubernetes authentication/authorization: cloud-controller-manager may need RBAC rules...
---

* cloud authentication/authorization: your cloud may require a token or IAM rules
to allow access to their APIs
* kubernetes authentication/authorization: cloud-controller-manager may need RBAC
rules set to speak to the kubernetes apiserver
* high availability: like kube-controller-manager, you may want a high available
setup for cloud controller manager using leader election (on by default).### Running cloud-controller-manager
Successfully running cloud-controller-manager requires some changes to your cluster configuration.
* `kubelet` and `kube-controller-manager` must be set according to the
user's usage of external CCM. If the user has an external CCM (not the internal cloud
controller loops in the Kubernetes Controller Manager), then `--cloud-provider=external`
must be specified. Otherwise, it should not be specified.
Keep in mind that setting up your cluster to use cloud controller manager will
change your cluster behaviour in a few ways:
* Components that specify `--cloud-provider=external` will add a taint
`node.cloudprovider.kubernetes.io/uninitialized` with an effect `NoSchedule`
during initialization. This marks the node as needing a second initialization
from an external controller before it can be scheduled work. Note that in the
event that cloud controller manager is not available, new nodes in the cluster
will be left unschedulable. The taint is important since the scheduler may
require cloud specific information about nodes such as their region or type
(high cpu, gpu, high memory, spot instance, etc).
* cloud information about nodes in the cluster will no longer be retrieved using
local metadata, but instead all API calls to retrieve node information will go
through cloud controller manager. This may mean you can restrict access to your
cloud API on the kubelets for better security. For larger clusters you may want
to consider if cloud controller manager will hit rate limits since it is now
responsible for almost all API calls to your cloud from within the cluster.
The cloud controller manager can implement:
* Node controller - responsible for updating kubernetes nodes using cloud APIs
and deleting kubernetes nodes that were deleted on your cloud.
* Service controller - responsible for loadbalancers on your cloud against
services of type LoadBalancer.
* Route controller - responsible for setting up network routes on your cloud