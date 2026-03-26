---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#13-standard
chunk_level: standard
chunk_type: table
heading: Feedback
token_count: 502
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
||
logs at or above this threshold go to stderr when writing to files and stderr (no effect when -logtostderr=true or -alsologtostderr=true)
|
|-v, --v int|
||
number for the log level verbosity
|
|--version version[=true]|
||
--version, --version=raw prints version information and quits; --version=vX.Y.Z... sets the reported version
|
|--vmodule pattern=N,...|
||
comma-separated list of pattern=N settings for file-filtered logging (only works for text log format)
|
|--write-config-to string|
||
If set, write the default configuration values to this file and exit.
|
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified December 21, 2025 at 5:13 PM PST: [Update component reference for v1.35 (13a9e56765)](https://github.com/kubernetes/website/commit/13a9e56765c61ebabc11dc3bc32ed7416837380f)
This page is automatically generated.
If you plan to report an issue with this page, mention that the page is auto-generated in your issue description. The fix may need to happen elsewhere in the Kubernetes project.