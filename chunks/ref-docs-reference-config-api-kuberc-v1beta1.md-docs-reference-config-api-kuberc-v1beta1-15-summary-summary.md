---
doc_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
chunk_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1#15-summary
chunk_level: summary
chunk_type: table
heading: `AllowlistEntry`
token_count: 127
summary: * [Preference](#kubectl-config-k8s-io-v1beta1-Preference) AllowlistEntry is an entry in the allowlist. For each allowlist item, at least one field must be nonempty. A struct with all empty fields is...
---

* [Preference](#kubectl-config-k8s-io-v1beta1-Preference)
AllowlistEntry is an entry in the allowlist. For each allowlist item, at
least one field must be nonempty. A struct with all empty fields is
considered a misconfiguration error. Each field is a criterion for
execution. If multiple fields are specified, then the criteria of all
specified fields must be met. That is, the result of an individual entry is
the logical AND of all checks corresponding to the specified fields within
the entry.
|Field|Description|
|`name`**[Required]**