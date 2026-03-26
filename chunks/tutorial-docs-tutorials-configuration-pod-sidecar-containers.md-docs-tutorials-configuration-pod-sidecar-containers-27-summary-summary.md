---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#27-summary
chunk_level: summary
chunk_type: prose
heading: Adopting built-in sidecar containers
token_count: 126
summary: If you experience issues when validating the feature, it may be an indication that one of the 3rd party tools or mutating webhooks are broken. When the `SidecarContainers` feature gate is enabled,...
---

If you experience issues when validating the feature, it may be an indication that one of the
3rd party tools or mutating webhooks are broken.
When the `SidecarContainers` feature gate is enabled, Pods gain a new field in their API.
If tools pass unknown fields as-is using various patching strategies to mutate a Pod object,
this will not be a problem. However, there are tools that will strip out unknown fields;
if you have those, they must be recompiled with the v1.28+ version of Kubernetes API client code.
The way to check this is to use the `kubectl describe pod`