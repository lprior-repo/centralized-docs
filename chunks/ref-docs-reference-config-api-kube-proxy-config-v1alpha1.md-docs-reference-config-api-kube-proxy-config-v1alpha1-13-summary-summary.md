---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#13-summary
chunk_level: summary
chunk_type: table
heading: `LoggingOptions`
token_count: 102
summary: ## `LoggingOptions` LoggingOptions can be used with ValidateAndApplyWithOptions to override certain global defaults. |Field|Description| |`ErrorStream`**[Required]**...
---

## `LoggingOptions`
LoggingOptions can be used with ValidateAndApplyWithOptions to override
certain global defaults.
|Field|Description|
|`ErrorStream`**[Required]**
[`io.Writer`](https://pkg.go.dev/io#Writer)|
ErrorStream can be used to override the os.Stderr default.
|
|`InfoStream`**[Required]**
[`io.Writer`](https://pkg.go.dev/io#Writer)|
InfoStream can be used to override the os.Stdout default.
|