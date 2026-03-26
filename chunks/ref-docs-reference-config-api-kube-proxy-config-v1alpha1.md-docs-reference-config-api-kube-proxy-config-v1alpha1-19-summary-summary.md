---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#19-summary
chunk_level: summary
chunk_type: table
heading: `TimeOrMetaDuration`
token_count: 122
summary: ## `TimeOrMetaDuration` **Appears in:** * [LoggingConfiguration](#LoggingConfiguration) TimeOrMetaDuration is present only for backwards compatibility for the flushFrequency field, and new fields...
---

## `TimeOrMetaDuration`
**Appears in:**
* [LoggingConfiguration](#LoggingConfiguration)
TimeOrMetaDuration is present only for backwards compatibility for the
flushFrequency field, and new fields should use metav1.Duration.
|Field|Description|
|`Duration`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
Duration holds the duration
|
|`-`**[Required]**
`bool`|
SerializeAsString controls whether the value is serialized as a string or an integer
|