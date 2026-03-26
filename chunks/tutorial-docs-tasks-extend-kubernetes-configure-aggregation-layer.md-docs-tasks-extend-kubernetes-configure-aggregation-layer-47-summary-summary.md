---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#47-summary
chunk_level: summary
chunk_type: prose
heading: Enable Kubernetes Apiserver flags
token_count: 78
summary: ``` `apiVersion: apiregistration.k8s.io/v1 kind: APIService ... spec: ... service: namespace: my-service-namespace name: my-service-name port: 1234 caBundle: \"Ci0tLS0tQk...&lt;base64-encoded PEM...
---

```
`apiVersion: apiregistration.k8s.io/v1
kind: APIService
...
spec:
...
service:
namespace: my-service-namespace
name: my-service-name
port: 1234
caBundle: "Ci0tLS0tQk...&lt;base64-encoded PEM bundle&gt;...tLS0K"
...
`
```