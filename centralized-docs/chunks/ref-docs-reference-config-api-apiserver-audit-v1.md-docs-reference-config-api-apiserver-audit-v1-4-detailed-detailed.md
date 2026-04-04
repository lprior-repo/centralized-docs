---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#4-detailed
chunk_level: detailed
chunk_type: table
heading: `Stage`
token_count: 867
summary: ## `Level` (Alias of `string`) **Appears in:** * [Event](#audit-k8s-io-v1-Event) * [PolicyRule](#audit-k8s-io-v1-PolicyRule) Level defines the amount of information logged during auditing ##...
---

## `Level`
(Alias of `string`)
**Appears in:**
* [Event](#audit-k8s-io-v1-Event)
* [PolicyRule](#audit-k8s-io-v1-PolicyRule)
Level defines the amount of information logged during auditing
## `ObjectReference`
**Appears in:**
* [Event](#audit-k8s-io-v1-Event)
ObjectReference contains enough information to let you inspect or modify the referred object.
|Field|Description|
|`resource`
`string`|No description provided.|
|`namespace`
`string`|No description provided.|
|`name`
`string`|No description provided.|
|`uid`
[`k8s.io/apimachinery/pkg/types.UID`](https://pkg.go.dev/k8s.io/apimachinery/pkg/types#UID)|No description provided.|
|`apiGroup`
`string`|
APIGroup is the name of the API group that contains the referred object.
The empty string represents the core API group.
|
|`apiVersion`
`string`|
APIVersion is the version of the API group that contains the referred object.
|
|`resourceVersion`
`string`|No description provided.|
|`subresource`
`string`|No description provided.|
## `PolicyRule`
**Appears in:**
* [Policy](#audit-k8s-io-v1-Policy)
PolicyRule maps requests based off metadata to an audit Level.
Requests must match the rules of every field (an intersection of rules).
|Field|Description|
|`level`**[Required]**
[`Level`](#audit-k8s-io-v1-Level)|
The Level that requests matching this rule are recorded at.
|
|`users`
`[]string`|
The users (by authenticated user name) this rule applies to.
An empty list implies every user.
|
|`userGroups`
`[]string`|
The user groups this rule applies to. A user is considered matching
if it is a member of any of the UserGroups.
An empty list implies every user group.
|
|`verbs`
`[]string`|
The verbs that match this rule.
An empty list implies every verb.
|
|`resources`
[`[]GroupResources`](#audit-k8s-io-v1-GroupResources)|
Resources that this rule matches. An empty list implies all kinds in all API groups.
|
|`namespaces`
`[]string`|
Namespaces that this rule matches.
The empty string "" matches non-namespaced resources.
An empty list implies every namespace.
|
|`nonResourceURLs`
`[]string`|
NonResourceURLs is a set of URL paths that should be audited.
`\*`s are allowed, but only as the full, final step in the path.
Examples:
* `/metrics` - Log requests for apiserver metrics
* `/healthz\*` - Log all health checks|
|`omitStages`
[`[]Stage`](#audit-k8s-io-v1-Stage)|
OmitStages is a list of stages for which no events are created. Note that this can also
be specified policy wide in which case the union of both are omitted.
An empty list means no restrictions will apply.
|
|`omitManagedFields`
`bool`|
OmitManagedFields indicates whether to omit the managed fields of the request
and response bodies from being written to the API audit log.
* a value of 'true' will drop the managed fields from the API audit log
* a value of 'false' indicates that the managed fileds should be included
in the API audit log
Note that the value, if specified, in this rule will override the global default
If a value is not specified then the global default specified in
Policy.OmitManagedFields will stand.|
## `Stage`
(Alias of `string`)
**Appears in:**
* [Event](#audit-k8s-io-v1-Event)
* [Policy](#audit-k8s-io-v1-Policy)
* [PolicyRule](#audit-k8s-io-v1-PolicyRule)
Stage defines the stages in request handling that audit events may be generated.