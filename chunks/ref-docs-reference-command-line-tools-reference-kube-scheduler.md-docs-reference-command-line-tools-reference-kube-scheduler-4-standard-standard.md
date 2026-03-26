---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#4-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 508
summary: '. Options are: kube=1.32..1.35(default:1.35) If the component is not specified, defaults to \"kube\" | |--feature-gates colonSeparatedMultimapStringString| || Comma-separated list of...
---

'.
Options are: kube=1.32..1.35(default:1.35)
If the component is not specified, defaults to "kube"
|
|--feature-gates colonSeparatedMultimapStringString|
||
Comma-separated list of component:key=value pairs that describe feature gates for alpha/experimental features of different components.
If the component is not specified, defaults to "kube". This flag can be repeatedly invoked. For example: --feature-gates 'wardle:featureA=true,wardle:featureB=false' --feature-gates 'kube:featureC=true'Options are:
kube:APIResponseCompression=true|false (BETA - default=true)
kube:APIServerIdentity=true|false (BETA - default=true)
kube:APIServingWithRoutine=true|false (ALPHA - default=false)
kube:AllAlpha=true|false (ALPHA - default=false)
kube:AllBeta=true|false (BETA - default=false)
kube:AllowParsingUserUIDFromCertAuth=true|false (BETA - default=true)
kube:AllowUnsafeMalformedObjectDeletion=true|false (ALPHA - default=false)
kube:AuthorizePodWebsocketUpgradeCreatePermission=true|false (BETA - default=true)
kube:CBORServingAndStorage=true|false (ALPHA - default=false)
kube:CPUManagerPolicyAlphaOptions=true|false (ALPHA - default=false)
kube:CPUManagerPolicyBetaOptions=true|false (BETA - default=true)
kube:CRDObservedGenerationTracking=true|false (BETA - default=false)
kube:CSIServiceAccountTokenSecrets=true|false (BETA - default=true)
kube:CSIVolumeHealth=true|false (ALPHA - default=false)
kube:ClearingNominatedNodeNameAfterBinding=true|false (BETA - default=true)
kube:ClientsAllowCBOR=true|false (ALPHA - default=false)
kube:ClientsPreferCBOR=true|false (ALPHA - default=false)
kube:CloudControllerManagerWatchBasedRoutesReconciliation=true|false (ALPHA - default=false)
kube:CloudControllerManagerWebhook=true|false (ALPHA - default=false)
kube:ClusterTrustBundle=true|false (BETA - default=false)
kube:ClusterTrustBundleProjection=true|false (BETA - default=false)
kube:ComponentFlagz=true|false (ALPHA - default=false)
kube:ComponentStatusz=true|false (ALPHA - default=false)