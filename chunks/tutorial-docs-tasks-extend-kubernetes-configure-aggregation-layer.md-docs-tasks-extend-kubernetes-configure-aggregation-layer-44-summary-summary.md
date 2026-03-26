---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#44-summary
chunk_level: summary
chunk_type: prose
heading: Enable Kubernetes Apiserver flags
token_count: 123
summary: ` apiVersion: apiregistration.k8s.io/v1 kind: APIService metadata: name: &lt;name of the registration object&gt; spec: group: &lt;API group name this extension apiserver hosts&gt; version: &lt;API...
---

`
apiVersion: apiregistration.k8s.io/v1
kind: APIService
metadata:
name: &lt;name of the registration object&gt;
spec:
group: &lt;API group name this extension apiserver hosts&gt;
version: &lt;API version this extension apiserver hosts&gt;
groupPriorityMinimum: &lt;priority this APIService for this group, see API documentation&gt;
versionPriority: &lt;prioritizes ordering of this version within a group, see API documentation&gt;
service:
namespace: &lt;namespace of the extension apiserver service&gt;