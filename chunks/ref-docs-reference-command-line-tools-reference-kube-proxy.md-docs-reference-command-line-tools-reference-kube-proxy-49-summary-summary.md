---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#49-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 128
summary: &lt;major&gt;.&lt;minor&gt;, e.g.: '1.16'. The purpose of this format is make sure you have the opportunity to notice if the next release hides additional metrics, rather than being surprised when...
---

&lt;major&gt;.&lt;minor&gt;, e.g.: '1.16'. The purpose of this format is make sure you have the opportunity to notice if the next release hides additional metrics, rather than being surprised when they are permanently removed in the release after that. This parameter is ignored if a config file is specified by --config.
|
|--skip\_headers|
||
If true, avoid header prefixes in the log messages
|
|--skip\_log\_headers|
||
If true, avoid headers when opening log files (no effect when -logtostderr=true)
|
|--stderrthreshold intDefault: 2|