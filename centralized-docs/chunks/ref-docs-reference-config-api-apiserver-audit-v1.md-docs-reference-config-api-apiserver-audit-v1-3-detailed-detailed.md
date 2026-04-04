---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#3-detailed
chunk_level: detailed
chunk_type: table
heading: `Level`
token_count: 986
summary: ## `EventList` EventList is a list of audit Events. |Field|Description| |`apiVersion` string|`audit.k8s.io/v1`| |`kind` string|`EventList`| |`metadata`...
---

## `EventList`
EventList is a list of audit Events.
|Field|Description|
|`apiVersion`
string|`audit.k8s.io/v1`|
|`kind`
string|`EventList`|
|`metadata`
[`meta/v1.ListMeta`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#listmeta-v1-meta)|No description provided.|
|`items`**[Required]**
[`[]Event`](#audit-k8s-io-v1-Event)|No description provided.|
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
## `AuthenticationMetadata`
**Appears in:**
* [Event](#audit-k8s-io-v1-Event)|Field|Description|
|`impersonationConstraint`
`string`|
ImpersonationConstraint is the verb associated with the constrained impersonation mode that was used to authorize
the ImpersonatedUser associated with this audit event. It is only set when constrained impersonation was used.
|
## `GroupResources`
**Appears in:**
* [PolicyRule](#audit-k8s-io-v1-PolicyRule)
GroupResources represents resource kinds in an API group.
|Field|Description|
|`group`
`string`|
Group is the name of the API group that contains the resources.
The empty string represents the core API group.
|
|`resources`
`[]string`|
Resources is a list of resources this rule applies to.
For example:
* `pods` matches pods.
* `pods/log` matches the log subresource of pods.
* `\*` matches all resources and their subresources.
* `pods/\*` matches all subresources of pods.
* `\*/scale` matches all scale subresources.
If wildcard is present, the validation rule will ensure resources do not
overlap with each other.
An empty list implies all resources and subresources in this API groups apply.
|
|`resourceNames`
`[]string`|
ResourceNames is a list of resource instance names that the policy matches.
Using this field requires Resources to be specified.
An empty list implies that every instance of the resource is matched.
|
## `Level`
(Alias of `string`)
**Appears in:**
* [Event](#audit-k8s-io-v1-Event)
* [PolicyRule](#audit-k8s-io-v1-PolicyRule)
Level defines the amount of information logged during auditing