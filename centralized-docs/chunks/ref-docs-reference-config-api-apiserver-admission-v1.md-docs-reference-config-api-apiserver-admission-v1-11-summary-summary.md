---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#11-summary
chunk_level: summary
chunk_type: prose
heading: `AdmissionRequest`
token_count: 125
summary: [`meta/v1.GroupVersionResource`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#GroupVersionResource)| requestResource is the fully-qualified resource of the original API request (for...
---

[`meta/v1.GroupVersionResource`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#GroupVersionResource)|
requestResource is the fully-qualified resource of the original API request (for example, v1.pods).
If this is specified and differs from the value in "resource", an equivalent match and conversion was performed.
For example, if deployments can be modified via apps/v1 and apps/v1beta1, and a webhook registered a rule of
`apiGroups:["apps"], apiVersions:["v1"], resources: ["deployments"]` and `matchPolicy: Equivalent`,