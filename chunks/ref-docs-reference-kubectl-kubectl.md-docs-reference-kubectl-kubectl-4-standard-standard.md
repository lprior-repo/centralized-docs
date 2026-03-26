---
doc_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl
chunk_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl#4-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 203
summary: |--skip-headers| ||If true, avoid header prefixes in the log messages| |--skip-log-headers| ||If true, avoid headers when opening log files| |--stderrthreshold severityDefault: 2| ||logs at or above...
---

|--skip-headers|
||If true, avoid header prefixes in the log messages|
|--skip-log-headers|
||If true, avoid headers when opening log files|
|--stderrthreshold severityDefault: 2|
||logs at or above this threshold go to stderr|
|--tls-server-name string|
||Server name to use for server certificate validation. If it is not provided, the hostname used to contact the server is used|
|--token string|
||Bearer token for authentication to the API server|
|--user string|
||The name of the kubeconfig user to use|
|--username string|
||Username for basic authentication to the API server|
|-v, --v Level|
||number for the log level verbosity|
|--version version[=true]|
||Print version information and quit|
|--vmodule moduleSpec|
||comma-separated list of pattern=N settings for file-filtered logging|
|--warnings-as-errors|
||Treat warnings received from the server as errors and exit with a non-zero exit code|