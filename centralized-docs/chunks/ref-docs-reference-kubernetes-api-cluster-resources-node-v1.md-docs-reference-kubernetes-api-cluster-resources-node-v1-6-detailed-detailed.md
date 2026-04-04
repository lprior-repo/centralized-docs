---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#6-detailed
chunk_level: detailed
chunk_type: prose
heading: NodeStatus
token_count: 1024
summary: * **config.lastKnownGood** (NodeConfigSource) LastKnownGood reports the checkpointed config the node will fall back to when it encounters an error attempting to use the Assigned config. The Assigned...
---

* **config.lastKnownGood** (NodeConfigSource)
LastKnownGood reports the checkpointed config the node will fall back to when it encounters an error attempting to use the Assigned config. The Assigned config becomes the LastKnownGood config when the node determines that the Assigned config is stable and correct. This is currently implemented as a 10-minute soak period starting when the local record of Assigned config is updated. If the Assigned config is Active at the end of this period, it becomes the LastKnownGood. Note that if Spec.ConfigSource is reset to nil (use local defaults), the LastKnownGood is also immediately reset to nil, because the local default config is always assumed good. You should not make assumptions about the node's method of determining config stability and correctness, as this may change or become configurable in the future.
*NodeConfigSource specifies a source of node configuration. Exactly one subfield (excluding metadata) must be non-nil. This API is deprecated since 1.22*
* **config.lastKnownGood.configMap** (ConfigMapNodeConfigSource)
ConfigMap is a reference to a Node's ConfigMap
*ConfigMapNodeConfigSource contains the information to reference a ConfigMap as a config source for the Node. This API is deprecated since 1.22: [https://git.k8s.io/enhancements/keps/sig-node/281-dynamic-kubelet-configuration](https://git.k8s.io/enhancements/keps/sig-node/281-dynamic-kubelet-configuration)*
* **config.lastKnownGood.configMap.kubeletConfigKey** (string), required
KubeletConfigKey declares which key of the referenced ConfigMap corresponds to the KubeletConfiguration structure This field is required in all cases.
* **config.lastKnownGood.configMap.name** (string), required
Name is the metadata.name of the referenced ConfigMap. This field is required in all cases.
* **config.lastKnownGood.configMap.namespace** (string), required
Namespace is the metadata.namespace of the referenced ConfigMap. This field is required in all cases.
* **config.lastKnownGood.configMap.resourceVersion** (string)
ResourceVersion is the metadata.ResourceVersion of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.
* **config.lastKnownGood.configMap.uid** (string)
UID is the metadata.UID of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.
* **daemonEndpoints** (NodeDaemonEndpoints)
Endpoints of daemons running on the Node.
*NodeDaemonEndpoints lists ports opened by daemons running on the Node.*
* **daemonEndpoints.kubeletEndpoint** (DaemonEndpoint)
Endpoint on which Kubelet is listening.
*DaemonEndpoint contains information about a single Daemon endpoint.*
* **daemonEndpoints.kubeletEndpoint.Port** (int32), required
Port number of the given endpoint.
* **declaredFeatures** ([]string)
*Atomic: will be replaced during a merge*
DeclaredFeatures represents the features related to feature gates that are declared by the node.
* **features** (NodeFeatures)
Features describes the set of features implemented by the CRI implementation.
*NodeFeatures describes the set of features implemented by the CRI implementation. The features contained in the NodeFeatures should depend only on the cri implementation independent of runtime handlers.*
* **features.supplementalGroupsPolicy** (boolean)
SupplementalGroupsPolicy is set to true if the runtime supports SupplementalGroupsPolicy and ContainerUser.
* **images** ([]ContainerImage)
*Atomic: will be replaced during a merge*
List of container images on this node
*Describe a container image*
* **images.names** ([]string)
*Atomic: will be replaced during a merge*
Names by which this image is known. e.g. ["kubernetes.example/hyperkube:v1.0.7", "cloud-vendor.registry.example/cloud-vendor/hyperkube:v1.0.7"]
* **images.sizeBytes** (int64)
The size of the image in bytes.
* **nodeInfo** (NodeSystemInfo)
Set of ids/uuids to uniquely identify the node. More info: [https://kubernetes.io/docs/reference/node/node-status/#info](https://kubernetes.io/docs/reference/node/node-status/#info)
*NodeSystemInfo is a set of ids/uuids to uniquely identify the node.*
* **nodeInfo.architecture** (string), required
The Architecture reported by the node
* **nodeInfo.bootID** (string), required
Boot ID reported by the node.
* **nodeInfo.containerRuntimeVersion** (string), required
ContainerRuntime Version reported by the node through runtime remote API (e.g. containerd://1.4.2).
* **nodeInfo.kernelVersion** (string), required
Kernel Version reported by the node from 'uname -r' (e.g. 3.16.0-0.bpo.4-amd64).