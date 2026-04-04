---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#6-standard
chunk_level: standard
chunk_type: table
heading: `Level`
token_count: 379
summary: ## `AuthenticationMetadata` **Appears in:** * [Event](#audit-k8s-io-v1-Event)|Field|Description| |`impersonationConstraint` `string`| ImpersonationConstraint is the verb associated with the...
---

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