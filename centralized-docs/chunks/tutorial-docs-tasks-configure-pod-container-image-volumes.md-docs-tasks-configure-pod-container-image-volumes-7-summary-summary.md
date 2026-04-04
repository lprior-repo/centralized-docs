---
doc_id: tutorial/docs-tasks-configure-pod-container-image-volumes.md/docs-tasks-configure-pod-container-image-volumes
chunk_id: tutorial/docs-tasks-configure-pod-container-image-volumes.md/docs-tasks-configure-pod-container-image-volumes#7-summary
chunk_level: summary
chunk_type: prose
heading: Use `subPath` (or `subPathExpr`)
token_count: 121
summary: ## Use `subPath` (or `subPathExpr`) It is possible to utilize [`subPath`](/docs/concepts/storage/volumes/#using-subpath) or...
---

## Use `subPath` (or `subPathExpr`)
It is possible to utilize
[`subPath`](/docs/concepts/storage/volumes/#using-subpath) or
[`subPathExpr`](/docs/concepts/storage/volumes/#using-subpath-expanded-environment)
from Kubernetes v1.33 when using the image volume feature.
[`pods/image-volumes-subpath.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/image-volumes-subpath.yaml)![](/images/copycode.svg "Copy pods/image-volumes-subpath.yaml to clipboard")