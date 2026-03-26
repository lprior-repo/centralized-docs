---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#2-detailed
chunk_level: detailed
chunk_type: table
heading: Options
token_count: 1021
summary: |--add\_dir\_header| || If true, adds the file directory to the header of the log messages | |--alsologtostderr| || log to standard error as well as files (no effect when -logtostderr=true) |...
---

|--add\_dir\_header|
||
If true, adds the file directory to the header of the log messages
|
|--alsologtostderr|
||
log to standard error as well as files (no effect when -logtostderr=true)
|
|--bind-address stringDefault: 0.0.0.0|
||
Overrides kube-proxy's idea of what its node's primary IP is. Note that the name is a historical artifact, and kube-proxy does not actually bind any sockets to this IP. This parameter is ignored if a config file is specified by --config.
|
|--bind-address-hard-fail|
||
If true kube-proxy will treat failure to bind to a port as fatal and exit
|
|--cleanup|
||
If true cleanup iptables and ipvs rules and exit.
|
|--cluster-cidr string|
||
The CIDR range of the pods in the cluster. (For dual-stack clusters, this can be a comma-separated dual-stack pair of CIDR ranges.). When --detect-local-mode is set to ClusterCIDR, kube-proxy will consider traffic to be local if its source IP is in this range. (Otherwise it is not used.) This parameter is ignored if a config file is specified by --config.
|
|--config string|
||
The path to the configuration file.
|
|--config-sync-period durationDefault: 15m0s|
||
How often configuration from the apiserver is refreshed. Must be greater than 0.
|
|--conntrack-max-per-core int32Default: 32768|
||
Maximum number of NAT connections to track per CPU core (0 to leave the limit as-is and ignore conntrack-min).
|
|--conntrack-min int32Default: 131072|
||
Minimum number of conntrack entries to allocate, regardless of conntrack-max-per-core (set conntrack-max-per-core=0 to leave the limit as-is).
|
|--conntrack-tcp-be-liberal|
||
Enable liberal mode for tracking TCP packets by setting nf\_conntrack\_tcp\_be\_liberal to 1
|
|--conntrack-tcp-timeout-close-wait durationDefault: 1h0m0s|
||
NAT timeout for TCP connections in the CLOSE\_WAIT state
|
|--conntrack-tcp-timeout-established durationDefault: 24h0m0s|
||
Idle timeout for established TCP connections (0 to leave as-is)
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
APIResponseCompression=true|false (BETA - default=true)
APIServerIdentity=true|false (BETA - default=true)
APIServingWithRoutine=true|false (ALPHA - default=false)
AllAlpha=true|false (ALPHA - default=false)
AllBeta=true|false (BETA - default=false)
AllowParsingUserUIDFromCertAuth=true|false (BETA - default=true)
AllowUnsafeMalformedObjectDeletion=true|false (ALPHA - default=false)
AuthorizePodWebsocketUpgradeCreatePermission=true|false (BETA - default=true)
CBORServingAndStorage=true|false (ALPHA - default=false)
CPUManagerPolicyAlphaOptions=true|false (ALPHA - default=false)
CPUManagerPolicyBetaOptions=true|false (BETA - default=true)
CRDObservedGenerationTracking=true|false (BETA - default=false)
CSIServiceAccountTokenSecrets=true|false (BETA - default=true)
CSIVolumeHealth=true|false (ALPHA - default=false)
ClearingNominatedNodeNameAfterBinding=true|false (BETA - default=true)
ClientsAllowCBOR=true|false (ALPHA - default=false)
ClientsPreferCBOR=true|false (ALPHA - default=false)
CloudControllerManagerWatchBasedRoutesReconciliation=true|false (ALPHA - default=false)
CloudControllerManagerWebhook=true|false (ALPHA - default=false)
ClusterTrustBundle=true|false (BETA - default=false)
ClusterTrustBundleProjection=true|false (BETA - default=false)
ComponentFlagz=true|false (ALPHA - default=false)
ComponentStatusz=true|false (ALPHA - default=false)
ConcurrentWatchObjectDecode=true|false (BETA - default=false)
ConstrainedImpersonation=true|false (ALPHA - default=false)
ContainerCheckpoint=true|false (BETA - default=true)
ContainerRestartRules=true|false (BETA - default=true)
ContainerStopSignals=true|false (ALPHA - default=false)