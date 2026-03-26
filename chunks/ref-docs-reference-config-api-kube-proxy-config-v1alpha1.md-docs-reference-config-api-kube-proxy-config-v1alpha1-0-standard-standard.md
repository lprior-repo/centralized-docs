---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#0-standard
chunk_level: standard
chunk_type: table
heading: Resource Types
token_count: 466
summary: ## Table of Contents    - [Resource Types](#resource-types)   - [`JSONOptions`](#jsonoptions)   - [`LogFormatFactory`](#logformatfactory)   - [`LoggingConfiguration`](#loggingconfiguration)   -...
---

## Table of Contents

  - [Resource Types](#resource-types)
  - [`JSONOptions`](#jsonoptions)
  - [`LogFormatFactory`](#logformatfactory)
  - [`LoggingConfiguration`](#loggingconfiguration)
  - [`LoggingOptions`](#loggingoptions)
  - [`OutputRoutingOptions`](#outputroutingoptions)
  - [`TextOptions`](#textoptions)
  - [`TimeOrMetaDuration`](#timeormetaduration)
  - [`VModuleConfiguration`](#vmoduleconfiguration)
  - [`VerbosityLevel`](#verbositylevel)
  - [`ClientConnectionConfiguration`](#clientconnectionconfiguration)
  - [`DebuggingConfiguration`](#debuggingconfiguration)
  - [`LeaderElectionConfiguration`](#leaderelectionconfiguration)
  - [`KubeProxyConfiguration`](#kubeproxyconfiguration)
  - [`DetectLocalConfiguration`](#detectlocalconfiguration)
  - [`KubeProxyConntrackConfiguration`](#kubeproxyconntrackconfiguration)
  - [`KubeProxyIPTablesConfiguration`](#kubeproxyiptablesconfiguration)
  - [`KubeProxyIPVSConfiguration`](#kubeproxyipvsconfiguration)
  - [`KubeProxyNFTablesConfiguration`](#kubeproxynftablesconfiguration)
  - [`KubeProxyWinkernelConfiguration`](#kubeproxywinkernelconfiguration)
  - [`LocalMode`](#localmode)
  - [`ProxyMode`](#proxymode)
  - [Feedback](#feedback)

---

## Resource Types
* [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration)## `FormatOptions`
**Appears in:**
* [LoggingConfiguration](#LoggingConfiguration)
FormatOptions contains options for the different logging formats.
|Field|Description|
|`text`**[Required]**
[`TextOptions`](#TextOptions)|
[Alpha] Text contains options for logging format "text".
Only available when the LoggingAlphaOptions feature gate is enabled.
|
|`json`**[Required]**
[`JSONOptions`](#JSONOptions)|
[Alpha] JSON contains options for logging format "json".
Only available when the LoggingAlphaOptions feature gate is enabled.
|