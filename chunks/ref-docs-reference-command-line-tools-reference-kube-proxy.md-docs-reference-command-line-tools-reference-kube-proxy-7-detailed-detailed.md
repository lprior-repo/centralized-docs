---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#7-detailed
chunk_level: detailed
chunk_type: table
heading: Feedback
token_count: 966
summary: \" / \"[::]:10249\" to bind on all interfaces.) Set empty to disable. This parameter is ignored if a config file is specified by --config. | |--nodeport-addresses strings| || A list of CIDR ranges that...
---

" / "[::]:10249" to bind on all interfaces.) Set empty to disable. This parameter is ignored if a config file is specified by --config.
|
|--nodeport-addresses strings|
||
A list of CIDR ranges that contain valid node IPs, or alternatively, the single string 'primary'. If set to a list of CIDRs, connections to NodePort services will only be accepted on node IPs in one of the indicated ranges. If set to 'primary', NodePort services will only be accepted on the node's primary IP(s) according to the Node object. If unset, NodePort connections will be accepted on all local IPs. This parameter is ignored if a config file is specified by --config.
|
|--one\_output|
||
If true, only write logs to their native severity level (vs also writing to each lower severity level; no effect when -logtostderr=true)
|
|--oom-score-adj int32Default: -999|
||
The oom-score-adj value for kube-proxy process. Values must be within the range [-1000, 1000]. This parameter is ignored if a config file is specified by --config.
|
|--pod-bridge-interface string|
||
A bridge interface name. When --detect-local-mode is set to BridgeInterface, kube-proxy will consider traffic to be local if it originates from this bridge.
|
|--pod-interface-name-prefix string|
||
An interface name prefix. When --detect-local-mode is set to InterfaceNamePrefix, kube-proxy will consider traffic to be local if it originates from any interface whose name begins with this prefix.
|
|--profiling|
||
If true enables profiling via web interface on /debug/pprof handler. This parameter is ignored if a config file is specified by --config.
|
|--proxy-mode ProxyMode|
||
Which proxy mode to use: on Linux this can be 'iptables' (default), 'ipvs', or 'nftables'. On Windows the only supported value is 'kernelspace'. This parameter is ignored if a config file is specified by --config.
|
|--show-hidden-metrics-for-version string|
||
The previous version for which you want to show hidden metrics. Only the previous minor version is meaningful, other values will not be allowed. The format is &lt;major&gt;.&lt;minor&gt;, e.g.: '1.16'. The purpose of this format is make sure you have the opportunity to notice if the next release hides additional metrics, rather than being surprised when they are permanently removed in the release after that. This parameter is ignored if a config file is specified by --config.
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