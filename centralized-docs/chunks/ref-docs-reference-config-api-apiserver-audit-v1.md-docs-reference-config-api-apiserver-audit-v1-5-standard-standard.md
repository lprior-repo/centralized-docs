---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#5-standard
chunk_level: standard
chunk_type: table
heading: `PolicyList`
token_count: 493
summary: ## `Policy` **Appears in:** * [PolicyList](#audit-k8s-io-v1-PolicyList) Policy defines the configuration of audit logging, and the rules for how different request categories are logged....
---

## `Policy`
**Appears in:**
* [PolicyList](#audit-k8s-io-v1-PolicyList)
Policy defines the configuration of audit logging, and the rules for how different request
categories are logged.
|Field|Description|
|`apiVersion`
string|`audit.k8s.io/v1`|
|`kind`
string|`Policy`|
|`metadata`
[`meta/v1.ObjectMeta`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#objectmeta-v1-meta)|
ObjectMeta is included for interoperability with API infrastructure.
Refer to the Kubernetes API documentation for the fields of the `metadata` field.|
|`rules`**[Required]**
[`[]PolicyRule`](#audit-k8s-io-v1-PolicyRule)|
Rules specify the audit Level a request should be recorded at.
A request may match multiple rules, in which case the FIRST matching rule is used.
The default audit level is None, but can be overridden by a catch-all rule at the end of the list.
PolicyRules are strictly ordered.
|
|`omitStages`
[`[]Stage`](#audit-k8s-io-v1-Stage)|
OmitStages is a list of stages for which no events are created. Note that this can also
be specified per rule in which case the union of both are omitted.
|
|`omitManagedFields`
`bool`|
OmitManagedFields indicates whether to omit the managed fields of the request
and response bodies from being written to the API audit log.
This is used as a global default - a value of 'true' will omit the managed fileds,
otherwise the managed fields will be included in the API audit log.
Note that this can also be specified per rule in which case the value specified
in a rule will override the global default.
|
## `PolicyList`
PolicyList is a list of audit Policies.
|Field|Description|
|`apiVersion`
string|`audit.k8s.io/v1`|
|`kind`
string|`PolicyList`|
|`metadata`
[`meta/v1.ListMeta`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#listmeta-v1-meta)|No description provided.|
|`items`**[Required]**
[`[]Policy`](#audit-k8s-io-v1-Policy)|No description provided.|