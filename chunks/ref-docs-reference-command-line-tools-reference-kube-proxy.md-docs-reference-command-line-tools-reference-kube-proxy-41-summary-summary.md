---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#41-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 126
summary: | |--log-text-split-stream| || [Alpha] In text format, write error messages to stderr and info messages to stdout. The default is to write a single stream to stdout. Enable the LoggingAlphaOptions...
---

|
|--log-text-split-stream|
||
[Alpha] In text format, write error messages to stderr and info messages to stdout. The default is to write a single stream to stdout. Enable the LoggingAlphaOptions feature gate to use this.
|
|--log\_backtrace\_at &lt;a string in the form 'file:N'&gt;Default: :0|
||
when logging hits line file:N, emit a stack trace
|
|--log\_dir string|
||
If non-empty, write log files in this directory (no effect when -logtostderr=true)
|
|--log\_file string|
||