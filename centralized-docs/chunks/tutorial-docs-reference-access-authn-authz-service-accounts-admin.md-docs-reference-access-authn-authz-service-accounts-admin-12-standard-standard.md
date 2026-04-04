---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#12-standard
chunk_level: standard
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 436
summary: ### ServiceAccount admission controller The modification of pods is implemented via a plugin called an [Admission Controller](/docs/reference/access-authn-authz/admission-controllers/). It is part of...
---

### ServiceAccount admission controller
The modification of pods is implemented via a plugin
called an [Admission Controller](/docs/reference/access-authn-authz/admission-controllers/).
It is part of the API server.
This admission controller acts synchronously to modify pods as they are created.
When this plugin is active (and it is by default on most distributions), then
it does the following when a Pod is created:
1. If the pod does not have a `.spec.serviceAccountName` set, the admission controller sets the name of the
ServiceAccount for this incoming Pod to `default`.
2. The admission controller ensures that the ServiceAccount referenced by the incoming Pod exists. If there
is no ServiceAccount with a matching name, the admission controller rejects the incoming Pod. That check
applies even for the `default` ServiceAccount.
3. Provided that neither the ServiceAccount's `automountServiceAccountToken` field nor the
Pod's `automountServiceAccountToken` field is set to `false`:
* the admission controller mutates the incoming Pod, adding an extra
[volume](/docs/concepts/storage/volumes/) that contains
a token for API access.
* the admission controller adds a `volumeMount` to each container in the Pod,
skipping any containers that already have a volume mount defined for the path
`/var/run/secrets/kubernetes.io/serviceaccount`.
For Linux containers, that volume is mounted at `/var/run/secrets/kubernetes.io/serviceaccount`;
on Windows nodes, the mount is at the equivalent path.
* If the spec of the incoming Pod doesn't already contain any `imagePullSecrets`, then the
admission controller adds `imagePullSecrets`, copying them from the `ServiceAccount`.### Legacy ServiceAccount token tracking controller
FEATURE STATE:
`Kubernetes v1.28 [stable]`(enabled by default)
This controller generates a ConfigMap called
`kube-system/kube-apiserver-legacy-service-account-token-tracking` in the
`kube-system` namespace. The ConfigMap records the timestamp when legacy service
account tokens began to be monitored by the system.