---
doc_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller
chunk_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller#7-standard
chunk_level: standard
chunk_type: prose
heading: Limitations
token_count: 500
summary: # these tolerations are to have the daemonset runnable on control plane nodes # remove them if your control plane nodes should not run pods - key: node-role.kubernetes.io/control-plane operator:...
---

# these tolerations are to have the daemonset runnable on control plane nodes
# remove them if your control plane nodes should not run pods
- key: node-role.kubernetes.io/control-plane
operator: Exists
effect: NoSchedule
- key: node-role.kubernetes.io/master
operator: Exists
effect: NoSchedule
# this is to restrict CCM to only run on master nodes
# the node selector may vary depending on your cluster setup
nodeSelector:
node-role.kubernetes.io/master: ""
`
```
## Limitations
Running cloud controller manager comes with a few possible limitations. Although
these limitations are being addressed in upcoming releases, it's important that
you are aware of these limitations for production workloads.
### Support for Volumes
Cloud controller manager does not implement any of the volume controllers found
in `kube-controller-manager` as the volume integrations also require coordination
with kubelets. As we evolve CSI (container storage interface) and add stronger
support for flex volume plugins, necessary support will be added to cloud
controller manager so that clouds can fully integrate with volumes. Learn more
about out-of-tree CSI volume plugins [here](https://github.com/kubernetes/features/issues/178).
### Scalability
The cloud-controller-manager queries your cloud provider's APIs to retrieve
information for all nodes. For very large clusters, consider possible
bottlenecks such as resource requirements and API rate limiting.
### Chicken and Egg
The goal of the cloud controller manager project is to decouple development
of cloud features from the core Kubernetes project. Unfortunately, many aspects
of the Kubernetes project has assumptions that cloud provider features are tightly
integrated into the project. As a result, adopting this new architecture can create
several situations where a request is being made for information from a cloud provider,
but the cloud controller manager may not be able to return that information without
the original request being complete.
A good example of this is the TLS bootstrapping feature in the Kubelet.
TLS bootstrapping assumes that the Kubelet has the ability to ask the cloud provider
(or a local metadata service) for all its address types (private, public, etc)
but cloud controller manager cannot set a node's address types without being
initialized in the first place which requires that the kubelet has TLS certificates
to communicate with the apiserver.
As this initiative evolves, changes will be made to address these issues in upcoming releases.