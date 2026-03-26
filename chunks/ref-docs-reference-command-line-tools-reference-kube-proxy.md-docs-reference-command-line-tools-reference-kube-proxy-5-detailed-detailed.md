---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#5-detailed
chunk_level: detailed
chunk_type: table
heading: Options
token_count: 1023
summary: StrictIPCIDRValidation=true|false (ALPHA - default=false) StructuredAuthenticationConfigurationEgressSelector=true|false (BETA - default=true)...
---

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
If false, kube-proxy will disable the legacy behavior of allowing NodePort services to be accessed via localhost. (Applies only to iptables mode and IPv4; localhost NodePorts are never allowed with other proxy modes or with IPv6.)
|
|--iptables-masquerade-bit int32Default: 14|
||
If using the iptables or ipvs proxy mode, the bit of the fwmark space to mark packets requiring SNAT with. Must be within the range [0, 31].
|
|--iptables-min-sync-period durationDefault: 1s|
||
The minimum period between iptables rule resyncs (e.g. '5s', '1m', '2h22m'). A value of 0 means every Service or EndpointSlice change will result in an immediate iptables resync.
|
|--iptables-sync-period durationDefault: 30s|
||
An interval (e.g. '5s', '1m', '2h22m') indicating how frequently various re-synchronizing and cleanup operations are performed. Must be greater than 0.
|
|--ipvs-exclude-cidrs strings|
||
A comma-separated list of CIDRs which the ipvs proxier should not touch when cleaning up IPVS rules.
|
|--ipvs-min-sync-period durationDefault: 1s|
||
The minimum period between IPVS rule resyncs (e.g. '5s', '1m', '2h22m'). A value of 0 means every Service or EndpointSlice change will result in an immediate IPVS resync.
|
|--ipvs-scheduler string|
||
The ipvs scheduler type when proxy mode is ipvs
|
|--ipvs-strict-arp|
||
Enable strict ARP by setting arp\_ignore to 1 and arp\_announce to 2
|
|--ipvs-sync-period durationDefault: 30s|
||
An interval (e.g. '5s', '1m', '2h22m') indicating how frequently various re-synchronizing and cleanup operations are performed. Must be greater than 0.
|
|--ipvs-tcp-timeout duration|
||
The timeout for idle IPVS TCP connections, 0 to leave as-is. (e.g. '5s', '1m', '2h22m').
|
|--ipvs-tcpfin-timeout duration|
||
The timeout for IPVS TCP connections after receiving a FIN packet, 0 to leave as-is. (e.g. '5s', '1m', '2h22m').
|
|--ipvs-udp-timeout duration|
||
The timeout for IPVS UDP packets, 0 to leave as-is. (e.g. '