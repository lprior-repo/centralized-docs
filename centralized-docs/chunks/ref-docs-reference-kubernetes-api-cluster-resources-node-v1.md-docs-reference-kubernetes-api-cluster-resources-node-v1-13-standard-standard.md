---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#13-standard
chunk_level: standard
chunk_type: prose
heading: NodeStatus
token_count: 501
summary: * **nodeInfo** (NodeSystemInfo) Set of ids/uuids to uniquely identify the node. More info:...
---

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
* **nodeInfo.kubeProxyVersion** (string), required
Deprecated: KubeProxy Version reported by the node.
* **nodeInfo.kubeletVersion** (string), required
Kubelet Version reported by the node.
* **nodeInfo.machineID** (string), required
MachineID reported by the node. For unique machine identification in the cluster this field is preferred. Learn more from man(5) machine-id: [http://man7.org/linux/man-pages/man5/machine-id.5.html](http://man7.org/linux/man-pages/man5/machine-id.5.html)
* **nodeInfo.operatingSystem** (string), required
The Operating System reported by the node
* **nodeInfo.osImage** (string), required
OS Image reported by the node from /etc/os-release (e.g. Debian GNU/Linux 7 (wheezy)).
* **nodeInfo.systemUUID** (string), required
SystemUUID reported by the node. For unique machine identification MachineID is preferred. This field is specific to Red Hat hosts [https://access.redhat.com/documentation/en-us/red\_hat\_subscription\_management/1/html/rhsm/uuid](https://access.redhat.com/documentation/en-us/red_hat_subscription_management/1/html/rhsm/uuid)
* **nodeInfo.swap** (NodeSwapStatus)
Swap Info reported by the node.
*NodeSwapStatus represents swap memory information.*
* **nodeInfo.swap.capacity** (int64)
Total amount of swap memory in bytes.