---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#3-detailed
chunk_level: detailed
chunk_type: table
heading: Options
token_count: 1021
summary: Version format could only be major.minor, for example: '--emulated-version=wardle=1.2,kube=1.31'. Options are: kube=1.32..1.35(default:1.35) If the component is not specified, defaults to \"kube\" |...
---

Version format could only be major.minor, for example: '--emulated-version=wardle=1.2,kube=1.31'.
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
kube:ConcurrentWatchObjectDecode=true|false (BETA - default=false)
kube:ConstrainedImpersonation=true|false (ALPHA - default=false)
kube:ContainerCheckpoint=true|false (BETA - default=true)
kube:ContainerRestartRules=true|false (BETA - default=true)
kube:ContainerStopSignals=true|false (ALPHA - default=false)
kube:ContextualLogging=true|false (BETA - default=true)
kube:CoordinatedLeaderElection=true|false (BETA - default=false)
kube:CrossNamespaceVolumeDataSource=true|false (ALPHA - default=false)
kube:CustomCPUCFSQuotaPeriod=true|false (ALPHA - default=false)
kube:DRAAdminAccess=true|false (BETA - default=true)
kube:DRAConsumableCapacity=true|false (ALPHA - default=false)
kube:DRADeviceBindingConditions=true|false (ALPHA - default=false)
kube:DRADeviceTaintRules=true|false (ALPHA - default=false)
kube:DRADeviceTaints=true|false (ALPHA - default=false)
kube:DRAExtendedResource=true|false (ALPHA - default=false)
kube:DRAPartitionableDevices=true|false (ALPHA - default=false)
kube:DRAPrioritizedList=true|false (BETA - default=true)
kube:DRAResourceClaimDeviceStatus=true|false (BETA - default=true)
kube:DRASchedulerFilterTimeout=true|false (BETA - default=true)
kube:DeclarativeValidation=true|false (BETA - default=true)
kube:DeclarativeValidationTakeover=true|false (BETA - default=false)
kube:DeploymentReplicaSetTerminatingReplicas=true|false (BETA - default=true)
kube:DetectCacheInconsistency=true|false (BETA - default=true)
kube:DisableCPUQuotaWithExclusiveCPUs=true|false (BETA - default=true)
kube:EnvFiles=true|false (BETA - default=true)
kube:EventedPLEG=true|false (ALPHA - default=false)
kube:ExternalServiceAccountTokenSigner=true|false (BETA - default=true)
kube:GangScheduling=true|false (ALPHA - default=false)
kube:GenericWorkload=true|false (ALPHA - default=false)