---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#18-summary
chunk_level: summary
chunk_type: prose
heading: Create a Pod that gets assigned a QoS class of Burstable
token_count: 128
summary: * The Pod does not meet the criteria for QoS class `Guaranteed`. * At least one Container in the Pod has a memory or CPU request or limit. Here is a manifest for a Pod that has one Container. The...
---

* The Pod does not meet the criteria for QoS class `Guaranteed`.
* At least one Container in the Pod has a memory or CPU request or limit.
Here is a manifest for a Pod that has one Container. The Container has a memory limit of 200 MiB
and a memory request of 100 MiB.
[`pods/qos/qos-pod-2.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/qos/qos-pod-2.yaml)![](/images/copycode.svg "Copy pods/qos/qos-pod-2.yaml to clipboard")