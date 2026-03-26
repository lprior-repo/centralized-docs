---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#11-summary
chunk_level: summary
chunk_type: prose
heading: `LoggingConfiguration`
token_count: 127
summary: | Maximum time between log flushes. If a string, parsed as a duration (i.e. \"1s\") If an int, the maximum number of nanoseconds (i.e. 1s = 1000000000). Ignored if the selected logging backend writes...
---

|
Maximum time between log flushes.
If a string, parsed as a duration (i.e. "1s")
If an int, the maximum number of nanoseconds (i.e. 1s = 1000000000).
Ignored if the selected logging backend writes log messages without buffering.
|
|`verbosity`**[Required]**
[`VerbosityLevel`](#VerbosityLevel)|
Verbosity is the threshold that determines which log messages are
logged. Default is zero which logs only the most important
messages. Higher values enable additional messages. Error messages
are always logged.
|
|`vmodule`