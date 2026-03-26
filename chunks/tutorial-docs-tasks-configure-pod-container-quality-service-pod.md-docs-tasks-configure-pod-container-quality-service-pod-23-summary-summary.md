---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#23-summary
chunk_level: summary
chunk_type: prose
heading: Create a Pod that gets assigned a QoS class of BestEffort
token_count: 117
summary: For a Pod to be given a QoS class of `BestEffort`, the Containers in the Pod must not have any memory or CPU limits or requests. Here is a manifest for a Pod that has one Container. The Container has...
---

For a Pod to be given a QoS class of `BestEffort`, the Containers in the Pod must not
have any memory or CPU limits or requests.
Here is a manifest for a Pod that has one Container. The Container has no memory or CPU
limits or requests:
[`pods/qos/qos-pod-3.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/qos/qos-pod-3.yaml)![](/images/copycode.svg "Copy pods/qos/qos-pod-3.yaml to clipboard")