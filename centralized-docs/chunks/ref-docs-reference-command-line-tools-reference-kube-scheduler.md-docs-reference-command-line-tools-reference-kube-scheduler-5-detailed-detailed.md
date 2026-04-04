---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#5-detailed
chunk_level: detailed
chunk_type: table
heading: Options
token_count: 1019
summary: kube:NodeLogQuery=true|false (BETA - default=false) kube:NominatedNodeNameForExpectation=true|false (BETA - default=true) kube:OpenAPIEnums=true|false (BETA - default=true)...
---

kube:NodeLogQuery=true|false (BETA - default=false)
kube:NominatedNodeNameForExpectation=true|false (BETA - default=true)
kube:OpenAPIEnums=true|false (BETA - default=true)
kube:OpportunisticBatching=true|false (BETA - default=true)
kube:PodAndContainerStatsFromCRI=true|false (ALPHA - default=false)
kube:PodCertificateRequest=true|false (BETA - default=false)
kube:PodDeletionCost=true|false (BETA - default=true)
kube:PodLevelResources=true|false (BETA - default=true)
kube:PodLogsQuerySplitStreams=true|false (ALPHA - default=false)
kube:PodReadyToStartContainersCondition=true|false (BETA - default=true)
kube:PodTopologyLabelsAdmission=true|false (BETA - default=true)
kube:PortForwardWebsockets=true|false (BETA - default=true)
kube:PreventStaticPodAPIReferences=true|false (BETA - default=true)
kube:ProcMountType=true|false (BETA - default=true)
kube:QOSReserved=true|false (ALPHA - default=false)
kube:ReduceDefaultCrashLoopBackOffDecay=true|false (ALPHA - default=false)
kube:RelaxedServiceNameValidation=true|false (ALPHA - default=false)
kube:ReloadKubeletServerCertificateFile=true|false (BETA - default=true)
kube:RemoteRequestHeaderUID=true|false (BETA - default=true)
kube:ResourceHealthStatus=true|false (ALPHA - default=false)
kube:RestartAllContainersOnContainerExits=true|false (ALPHA - default=false)
kube:RotateKubeletServerCertificate=true|false (BETA - default=true)
kube:RuntimeClassInImageCriApi=true|false (ALPHA - default=false)
kube:SELinuxChangePolicy=true|false (BETA - default=true)
kube:SELinuxMount=true|false (BETA - default=false)
kube:SELinuxMountReadWriteOncePod=true|false (BETA - default=true)
kube:SchedulerAsyncAPICalls=true|false (BETA - default=true)
kube:SchedulerAsyncPreemption=true|false (BETA - default=true)
kube:SchedulerPopFromBackoffQ=true|false (BETA - default=true)
kube:ServiceAccountNodeAudienceRestriction=true|false (BETA - default=true)
kube:SizeBasedListCostEstimate=true|false (BETA - default=true)
kube:StatefulSetSemanticRevisionComparison=true|false (BETA - default=true)
kube:StorageCapacityScoring=true|false (ALPHA - default=false)
kube:StorageVersionAPI=true|false (ALPHA - default=false)
kube:StorageVersionHash=true|false (BETA - default=true)
kube:StorageVersionMigrator=true|false (BETA - default=false)
kube:StrictIPCIDRValidation=true|false (ALPHA - default=false)
kube:StructuredAuthenticationConfigurationEgressSelector=true|false (BETA - default=true)
kube:StructuredAuthenticationConfigurationJWKSMetrics=true|false (BETA - default=true)
kube:TaintTolerationComparisonOperators=true|false (ALPHA - default=false)
kube:TokenRequestServiceAccountUIDValidation=true|false (BETA - default=true)
kube:TopologyManagerPolicyAlphaOptions=true|false (ALPHA - default=false)
kube:TopologyManagerPolicyBetaOptions=true|false (BETA - default=true)
kube:TranslateStreamCloseWebsocketRequests=true|false (BETA - default=true)
kube:UnauthenticatedHTTP2DOSMitigation=true|false (BETA - default=true)
kube:UnknownVersionInteroperabilityProxy=true|false (ALPHA - default=false)
kube:UserNamespacesHostNetworkSupport=true|false (ALPHA - default=false)
kube:UserNamespacesSupport=true|false (BETA - default=true)
kube:VolumeLimitScaling=true|false (ALPHA - default=false)
kube:WatchCacheInitializationPostStartHook=true|false (BETA - default=false)
kube:WatchList=true|false (BETA - default=true)
kube:WatchListClient=true|false (BETA - default=true)
kube:WindowsCPUAndMemoryAffinity=true|false (ALPHA - default=false)
kube:WindowsGracefulNodeShutdown=true|false (BETA - default=true)
|
|-h, --help|
||
help for kube-scheduler
|
|--http2-max-streams-per-connection int|
||
The limit that the server gives to clients for the maximum number of streams in an HTTP/2 connection. Zero means to use golang's default.
|
|--kube-api-burst int32Default: 100|
||
DEPRECATED: burst to use while talking with kubernetes apiserver. This parameter is ignored if a config file is specified in --config.
|
|--