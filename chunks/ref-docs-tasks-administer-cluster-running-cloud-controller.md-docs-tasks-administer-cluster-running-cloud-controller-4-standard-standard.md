---
doc_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller
chunk_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller#4-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 235
summary: * Node controller - responsible for updating kubernetes nodes using cloud APIs and deleting kubernetes nodes that were deleted on your cloud. * Service controller - responsible for loadbalancers on...
---

* Node controller - responsible for updating kubernetes nodes using cloud APIs
and deleting kubernetes nodes that were deleted on your cloud.
* Service controller - responsible for loadbalancers on your cloud against
services of type LoadBalancer.
* Route controller - responsible for setting up network routes on your cloud
* any other features you would like to implement if you are running an out-of-tree provider.## Examples
If you are using a cloud that is currently supported in Kubernetes core and would
like to adopt cloud controller manager, see the
[cloud controller manager in kubernetes core](https://github.com/kubernetes/kubernetes/tree/master/cmd/cloud-controller-manager).
For cloud controller managers not in Kubernetes core, you can find the respective
projects in repositories maintained by cloud vendors or by SIGs.
For providers already in Kubernetes core, you can run the in-tree cloud controller
manager as a DaemonSet in your cluster, use the following as a guideline:
[`admin/cloud/ccm-example.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/admin/cloud/ccm-example.yaml)![](/images/copycode.svg "Copy admin/cloud/ccm-example.yaml to clipboard")