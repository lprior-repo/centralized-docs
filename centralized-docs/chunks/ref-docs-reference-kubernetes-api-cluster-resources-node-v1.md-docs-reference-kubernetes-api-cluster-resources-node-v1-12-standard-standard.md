---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#12-standard
chunk_level: standard
chunk_type: prose
heading: NodeStatus
token_count: 493
summary: * **config.lastKnownGood.configMap.resourceVersion** (string) ResourceVersion is the metadata.ResourceVersion of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in...
---

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