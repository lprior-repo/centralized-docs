---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#3-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 508
summary: \_conntrack\_tcp\_be\_liberal to 1 | |--conntrack-tcp-timeout-close-wait durationDefault: 1h0m0s| || NAT timeout for TCP connections in the CLOSE\_WAIT state | |--conntrack-tcp-timeout-established...
---

\_conntrack\_tcp\_be\_liberal to 1
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