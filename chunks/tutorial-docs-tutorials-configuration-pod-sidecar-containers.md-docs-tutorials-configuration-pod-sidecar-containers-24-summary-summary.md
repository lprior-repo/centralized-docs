---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#24-summary
chunk_level: summary
chunk_type: table
heading: Adopting built-in sidecar containers
token_count: 122
summary: You should ensure that the feature gate is enabled for the API server(s) within the control plane **and** for all nodes. One of the ways to check the feature gate enablement is to run a command like...
---

You should ensure that the feature gate is enabled for the API server(s) within the control plane
**and** for all nodes.
One of the ways to check the feature gate enablement is to run a command like this:
* For API Server:
```
`kubectl get --raw /metrics | grep kubernetes\_feature\_enabled | grep SidecarContainers
`
```
* For the individual node:
```
`kubectl get --raw /api/v1/nodes/&lt;node-name&gt;/proxy/metrics | grep kubernetes\_feature\_enabled | grep SidecarContainers
`
```