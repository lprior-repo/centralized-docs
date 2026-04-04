---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#13-standard
chunk_level: standard
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 349
summary: ### Legacy ServiceAccount token cleaner FEATURE STATE: `Kubernetes v1.30 [stable]`(enabled by default) The legacy ServiceAccount token cleaner runs as part of the `kube-controller-manager` and checks...
---

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