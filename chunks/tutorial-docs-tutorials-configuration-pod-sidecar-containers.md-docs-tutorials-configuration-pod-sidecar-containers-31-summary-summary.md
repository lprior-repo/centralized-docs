---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#31-summary
chunk_level: summary
chunk_type: prose
heading: Adopting built-in sidecar containers
token_count: 75
summary: * query node version and assume the feature gate is enabled on the version 1.29+ * query node prometheus metrics and check feature enablement status * assume the nodes are running with a [supported...
---

* query node version and assume the feature gate is enabled on the version 1.29+
* query node prometheus metrics and check feature enablement status
* assume the nodes are running with a [supported version skew](/releases/version-skew-policy/#supported-version-skew)
from the API server
* there may be other custom ways to detect nodes compatibility.