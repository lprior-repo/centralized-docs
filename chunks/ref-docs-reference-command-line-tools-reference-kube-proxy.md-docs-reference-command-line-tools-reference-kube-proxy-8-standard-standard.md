---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#8-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 488
summary: StorageVersionMigrator=true|false (BETA - default=false) StrictIPCIDRValidation=true|false (ALPHA - default=false) StructuredAuthenticationConfigurationEgressSelector=true|false (BETA - default=true)...
---

StorageVersionMigrator=true|false (BETA - default=false)
StrictIPCIDRValidation=true|false (ALPHA - default=false)
StructuredAuthenticationConfigurationEgressSelector=true|false (BETA - default=true)
StructuredAuthenticationConfigurationJWKSMetrics=true|false (BETA - default=true)
TaintTolerationComparisonOperators=true|false (ALPHA - default=false)
TokenRequestServiceAccountUIDValidation=true|false (BETA - default=true)
TopologyManagerPolicyAlphaOptions=true|false (ALPHA - default=false)
TopologyManagerPolicyBetaOptions=true|false (BETA - default=true)
TranslateStreamCloseWebsocketRequests=true|false (BETA - default=true)
UnauthenticatedHTTP2DOSMitigation=true|false (BETA - default=true)
UnknownVersionInteroperabilityProxy=true|false (ALPHA - default=false)
UserNamespacesHostNetworkSupport=true|false (ALPHA - default=false)
UserNamespacesSupport=true|false (BETA - default=true)
VolumeLimitScaling=true|false (ALPHA - default=false)
WatchCacheInitializationPostStartHook=true|false (BETA - default=false)
WatchList=true|false (BETA - default=true)
WatchListClient=true|false (BETA - default=true)
WindowsCPUAndMemoryAffinity=true|false (ALPHA - default=false)
WindowsGracefulNodeShutdown=true|false (BETA - default=true)
This parameter is ignored if a config file is specified by --config.
|
|--healthz-bind-address ipportDefault: 0.0.0.0:10256|
||
The IP address and port for the health check server to serve on, defaulting to "0.0.0.0:10256". This parameter is ignored if a config file is specified by --config.
|
|-h, --help|
||
help for kube-proxy
|
|--hostname-override string|
||
If non-empty, will be used as the name of the Node that kube-proxy is running on. If unset, the node name is assumed to be the same as the node's hostname.
|
|--init-only|
||
If true, perform any initialization steps that must be done with full root privileges, and then exit. After doing this, you can run kube-proxy again with only the CAP\_NET\_ADMIN capability.
|
|--iptables-localhost-nodeportsDefault: true|
||