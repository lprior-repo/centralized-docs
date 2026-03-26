---
doc_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
chunk_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1#3-summary
chunk_level: summary
chunk_type: prose
heading: Resource Types
token_count: 128
summary: | defaults allow changing default option values of commands. This is especially useful, when user doesn't want to explicitly set options each time. | |`aliases`**[Required]**...
---

|
defaults allow changing default option values of commands.
This is especially useful, when user doesn't want to explicitly
set options each time.
|
|`aliases`**[Required]**
[`[]AliasOverride`](#kubectl-config-k8s-io-v1beta1-AliasOverride)|
aliases allow defining command aliases for existing kubectl commands, with optional default option values.
If the alias name collides with a built-in command, built-in command always takes precedence.
Option overrides defined in the defaults section do NOT apply to aliases for the same command.
kubectl [ALIAS NAME] [USER\_OPTIONS] [USER\_EXPLICIT