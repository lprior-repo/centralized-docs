---
doc_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod
chunk_id: tutorial/docs-tasks-configure-pod-container-quality-service-pod.md/docs-tasks-configure-pod-container-quality-service-pod#26-summary
chunk_level: summary
chunk_type: prose
heading: Create a Pod that has two Containers
token_count: 105
summary: ## Create a Pod that has two Containers Here is a manifest for a Pod that has two Containers. One container specifies a memory request of 200 MiB. The other Container does not specify any requests or...
---

## Create a Pod that has two Containers
Here is a manifest for a Pod that has two Containers. One container specifies a memory
request of 200 MiB. The other Container does not specify any requests or limits.
[`pods/qos/qos-pod-4.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/qos/qos-pod-4.yaml)![](/images/copycode.svg "Copy pods/qos/qos-pod-4.yaml to clipboard")