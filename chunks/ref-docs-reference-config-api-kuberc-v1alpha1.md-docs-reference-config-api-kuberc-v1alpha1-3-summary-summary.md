---
doc_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1
chunk_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1#3-summary
chunk_level: summary
chunk_type: prose
heading: Resource Types
token_count: 128
summary: | overrides allows changing default flag values of commands. This is especially useful, when user doesn't want to explicitly set flags each time. | |`aliases`**[Required]**...
---

|
overrides allows changing default flag values of commands.
This is especially useful, when user doesn't want to explicitly
set flags each time.
|
|`aliases`**[Required]**
[`[]AliasOverride`](#kubectl-config-k8s-io-v1alpha1-AliasOverride)|
aliases allow defining command aliases for existing kubectl commands, with optional default flag values.
If the alias name collides with a built-in command, built-in command always takes precedence.
Flag overrides defined in the overrides section do NOT apply to aliases for the same command.
kubectl [ALIAS NAME] [USER\_FLAGS] [USER\_EXPLICIT