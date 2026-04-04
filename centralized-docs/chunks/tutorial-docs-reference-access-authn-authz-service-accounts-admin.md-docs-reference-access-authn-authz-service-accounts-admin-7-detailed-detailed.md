---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#7-detailed
chunk_level: detailed
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 785
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
### Legacy ServiceAccount token cleaner
FEATURE STATE:
`Kubernetes v1.30 [stable]`(enabled by default)
The legacy ServiceAccount token cleaner runs as part of the
`kube-controller-manager` and checks every 24 hours to see if any auto-generated
legacy ServiceAccount token has not been used in a *specified amount of time*.
If so, the cleaner marks those tokens as invalid.
The cleaner works by first checking the ConfigMap created by the control plane
(provided that `LegacyServiceAccountTokenTracking` is enabled). If the current
time is a *specified amount of time* after the date in the ConfigMap, the
cleaner then loops through the list of Secrets in the cluster and evaluates each
Secret that has the type `kubernetes.io/service-account-token`.
If a Secret meets all of the following conditions, the cleaner marks it as
invalid:
* The Secret is auto-generated, meaning that it is bi-directionally referenced
by a ServiceAccount.
* The Secret is not currently mounted by any pods.
* The Secret has not been used in a *specified amount of time* since it was
created or since it was last used.
The cleaner marks a Secret invalid by adding a label called
`kubernetes.io/legacy-token-invalid-since` to the Secret, with the current date
as the value. If an invalid Secret is not used in a *specified amount of time*,
the cleaner will delete it.
#### Note:
All the *specified amount of time* above defaults to one year. The cluster
administrator can configure this value through the
`--legacy-service-account-token-clean-up-period` command line argument for the
`kube-controller-manager` component.