---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#8-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 123
summary: | |--conntrack-udp-timeout duration| || Idle timeout for UNREPLIED UDP connections (0 to leave as-is) | |--conntrack-udp-timeout-stream duration| || Idle timeout for ASSURED UDP connections (0 to...
---

|
|--conntrack-udp-timeout duration|
||
Idle timeout for UNREPLIED UDP connections (0 to leave as-is)
|
|--conntrack-udp-timeout-stream duration|
||
Idle timeout for ASSURED UDP connections (0 to leave as-is)
|
|--detect-local-mode LocalMode|
||
Mode to use to detect local traffic. This parameter is ignored if a config file is specified by --config.
|
|--feature-gates &lt;comma-separated 'key=True|False' pairs&gt;|
||
A set of key=value pairs that describe feature gates for alpha/experimental features. Options are: