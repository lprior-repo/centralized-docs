---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#10-standard
chunk_level: standard
chunk_type: prose
heading: What's next
token_count: 474
summary: ### Configurations to avoid Avoid the following configurations: 1. duplicates: the same NUMA node or memory type, but with a different value; 2. setting a zero limit for any of memory types; 3. NUMA...
---

### Configurations to avoid
Avoid the following configurations:
1. duplicates: the same NUMA node or memory type, but with a different value;
2. setting a zero limit for any of memory types;
3. NUMA node IDs that do not exist in the machine hardware;
4. memory type names different than `memory` or `hugepages-&lt;size&gt;`
(hugepages of particular `&lt;size&gt;` should also exist).## Placing a Pod in the Guaranteed QoS class
If the selected policy is anything other than `None`, the Memory Manager identifies pods
that are in the `Guaranteed` QoS class.
The Memory Manager provides specific topology hints to the Topology Manager for each `Guaranteed` pod.
For pods in a QoS class other than `Guaranteed`, the Memory Manager provides default topology hints
to the Topology Manager.
The following excerpts from pod manifests assign a pod to the `Guaranteed` QoS class.
A Pod with integer CPU(s) runs in the `Guaranteed` QoS class, when `requests` are equal to `limits`:
```
`spec:
containers:
- name: nginx
image: nginx
resources:
limits:
memory: "200Mi"
cpu: "2"
example.com/device: "1"
requests:
memory: "200Mi"
cpu: "2"
example.com/device: "1"
`
```
Also, a pod sharing CPU(s) runs in the `Guaranteed` QoS class, when `requests` are equal to `limits`.
```
`spec:
containers:
- name: nginx
image: nginx
resources:
limits:
memory: "200Mi"
cpu: "300m"
example.com/device: "1"
requests:
memory: "200Mi"
cpu: "300m"
example.com/device: "1"
`
```
Notice that both CPU and memory requests must be specified for a Pod to lend it to Guaranteed QoS class.
## What's next
* Read [Troubleshooting Topology Management](/docs/tasks/debug/debug-cluster/topology/)
* Read the [KEP](https://github.com/kubernetes/enhancements/tree/master/keps/sig-node/1769-memory-manager) (Kubernetes enhancement proposal) for memory manager