---
doc_id: tutorial/docs-tasks-configure-pod-container-attach-handler-lifecycle-event.md/docs-tasks-configure-pod-container-attach-handler-lifecycle-event
chunk_id: tutorial/docs-tasks-configure-pod-container-attach-handler-lifecycle-event.md/docs-tasks-configure-pod-container-attach-handler-lifecycle-event#4-summary
chunk_level: summary
chunk_type: prose
heading: Define postStart and preStop handlers
token_count: 90
summary: ## Define postStart and preStop handlers In this exercise, you create a Pod that has one Container. The Container has handlers for the postStart and preStop events. Here is the configuration file for...
---

## Define postStart and preStop handlers
In this exercise, you create a Pod that has one Container. The Container has handlers
for the postStart and preStop events.
Here is the configuration file for the Pod:
[`pods/lifecycle-events.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/lifecycle-events.yaml)![](/images/copycode.svg "Copy pods/lifecycle-events.yaml to clipboard")