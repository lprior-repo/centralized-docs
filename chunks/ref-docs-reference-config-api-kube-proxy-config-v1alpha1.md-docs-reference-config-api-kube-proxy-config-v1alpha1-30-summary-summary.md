---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#30-summary
chunk_level: summary
chunk_type: table
heading: `LeaderElectionConfiguration`
token_count: 104
summary: * [GenericControllerManagerConfiguration](#controllermanager-config-k8s-io-v1alpha1-GenericControllerManagerConfiguration) LeaderElectionConfiguration defines the configuration of leader election...
---

* [GenericControllerManagerConfiguration](#controllermanager-config-k8s-io-v1alpha1-GenericControllerManagerConfiguration)
LeaderElectionConfiguration defines the configuration of leader election
clients for components that can run with leader election enabled.
|Field|Description|
|`leaderElect`**[Required]**
`bool`|
leaderElect enables a leader election client to gain leadership
before executing the main loop. Enable this when running replicated
components for high availability.
|
|`leaseDuration`**[Required]**