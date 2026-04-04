---
doc_id: tutorial/docs-tasks-configure-pod-container-image-volumes.md/docs-tasks-configure-pod-container-image-volumes
chunk_id: tutorial/docs-tasks-configure-pod-container-image-volumes.md/docs-tasks-configure-pod-container-image-volumes#4-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 126
summary: * You need to enable the `ImageVolume` [feature gate](/docs/reference/command-line-tools-reference/feature-gates/)## Run a Pod that uses an image volume An image volume for a pod is enabled by...
---

* You need to enable the `ImageVolume` [feature gate](/docs/reference/command-line-tools-reference/feature-gates/)## Run a Pod that uses an image volume
An image volume for a pod is enabled by setting the `volumes[\*].image` field of `.spec`
to a valid reference and consuming it in the `volumeMounts` of the container. For example:
[`pods/image-volumes.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/image-volumes.yaml)![](/images/copycode.svg "Copy pods/image-volumes.yaml to clipboard")