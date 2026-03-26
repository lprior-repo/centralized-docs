---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#16-summary
chunk_level: summary
chunk_type: table
heading: `OutputRoutingOptions`
token_count: 102
summary: * [TextOptions](#TextOptions) OutputRoutingOptions contains options that are supported by both \"text\" and \"json\". |Field|Description| |`splitStream`**[Required]** `bool`| [Alpha] SplitStream...
---

* [TextOptions](#TextOptions)
OutputRoutingOptions contains options that are supported by both "text" and "json".
|Field|Description|
|`splitStream`**[Required]**
`bool`|
[Alpha] SplitStream redirects error messages to stderr while
info messages go to stdout, with buffering. The default is to write
both to stdout, without buffering. Only available when
the LoggingAlphaOptions feature gate is enabled.
|
|`infoBufferSize`**[Required]**