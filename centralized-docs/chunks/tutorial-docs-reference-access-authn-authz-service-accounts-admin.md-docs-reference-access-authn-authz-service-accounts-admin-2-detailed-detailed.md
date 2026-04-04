---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#2-detailed
chunk_level: detailed
chunk_type: prose
heading: User accounts versus service accounts
token_count: 654
summary: ## User accounts versus service accounts Kubernetes distinguishes between the concept of a user account and a service account for a number of reasons: * User accounts are for humans. Service accounts...
---

## User accounts versus service accounts
Kubernetes distinguishes between the concept of a user account and a service account
for a number of reasons:
* User accounts are for humans. Service accounts are for application processes,
which (for Kubernetes) run in containers that are part of pods.
* User accounts are intended to be global: names must be unique across all
namespaces of a cluster. No matter what namespace you look at, a particular
username that represents a user represents the same user.
In Kubernetes, service accounts are namespaced: two different namespaces can
contain ServiceAccounts that have identical names.
* Typically, a cluster's user accounts might be synchronised from a corporate
database, where new user account creation requires special privileges and is
tied to complex business processes. By contrast, service account creation is
intended to be more lightweight, allowing cluster users to create service accounts
for specific tasks on demand. Separating ServiceAccount creation from the steps to
onboard human users makes it easier for workloads to follow the principle of
least privilege.
* Auditing considerations for humans and service accounts may differ; the separation
makes that easier to achieve.
* A configuration bundle for a complex system may include definition of various service
accounts for components of that system. Because service accounts can be created
without many constraints and have namespaced names, such configuration is
usually portable.## Bound service account tokens
ServiceAccount tokens can be bound to API objects that exist in the kube-apiserver.
This can be used to tie the validity of a token to the existence of another API object.
Supported object types are as follows:
* Pod (used for projected volume mounts, see below)
* Secret (can be used to allow revoking a token by deleting the Secret)
* Node (can be used to auto-revoke a token when its Node is deleted; creating new node-bound tokens is GA in v1.33+)
When a token is bound to an object, the object's `metadata.name` and `metadata.uid` are
stored as extra 'private claims' in the issued JWT.
When a bound token is presented to the kube-apiserver, the service account authenticator
will extract and verify these claims.
If the referenced object or the ServiceAccount is pending deletion (for example, due to finalizers),
then for any instant that is 60 seconds (or more) after the `.metadata.deletionTimestamp` date,
authentication with that token would fail.
If the referenced object no longer exists (or its `metadata.uid` does not match),
the request will not be authenticated.
### Additional metadata in Pod bound tokens
FEATURE STATE:
`Kubernetes v1.32 [stable]`(enabled by default)
When a service account token is bound to a Pod object, additional metadata is also
embedded into the token that indicates the value of the bound pod's `spec.nodeName` field,
and the uid of that Node, if available.
This node information is **not** verified by the kube-apiserver when the token is used for authentication.
It is included so integrators do not have to fetch Pod or Node API objects to check the associated Node name
and uid when inspecting a JWT.