---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#44-summary
chunk_level: summary
chunk_type: prose
heading: Taint based Evictions
token_count: 122
summary: * `node.cloudprovider.kubernetes.io/uninitialized`: When the kubelet is started with an \"external\" cloud provider, this taint is set on a node to mark it as unusable. After a controller from the...
---

* `node.cloudprovider.kubernetes.io/uninitialized`: When the kubelet is started
with an "external" cloud provider, this taint is set on a node to mark it
as unusable. After a controller from the cloud-controller-manager initializes
this node, the kubelet removes this taint.
In case a node is to be drained, the node controller or the kubelet adds relevant taints
with `NoExecute` effect. This effect is added by default for the
`node.kubernetes.io/not-ready` and `node.kubernetes.io/unreachable` taints.