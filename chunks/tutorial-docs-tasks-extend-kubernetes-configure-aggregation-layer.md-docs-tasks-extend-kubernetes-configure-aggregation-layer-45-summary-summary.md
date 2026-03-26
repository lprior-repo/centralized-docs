---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#45-summary
chunk_level: summary
chunk_type: prose
heading: Enable Kubernetes Apiserver flags
token_count: 89
summary: service: namespace: &lt;namespace of the extension apiserver service&gt; name: &lt;name of the extension apiserver service&gt; caBundle: &lt;pem encoded ca cert that signs the server cert used by the...
---

service:
namespace: &lt;namespace of the extension apiserver service&gt;
name: &lt;name of the extension apiserver service&gt;
caBundle: &lt;pem encoded ca cert that signs the server cert used by the webhook&gt;
`
```
The name of an APIService object must be a valid
[path segment name](/docs/concepts/overview/working-with-objects/names/#path-segment-names).