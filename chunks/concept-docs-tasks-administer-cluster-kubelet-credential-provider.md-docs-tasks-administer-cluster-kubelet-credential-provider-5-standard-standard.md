---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#5-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 502
summary: - [pod-specific claims, then the plugin can set this to \"ServiceAccount\". In this case, the](#pod-specific-claims-then-the-plugin-can-set-this-to-serviceaccount-in-this-case-the) - [kubelet will...
---

- [pod-specific claims, then the plugin can set this to "ServiceAccount". In this case, the](#pod-specific-claims-then-the-plugin-can-set-this-to-serviceaccount-in-this-case-the)
- [kubelet will cache returned credentials on a per-serviceaccount basis. Use this when the](#kubelet-will-cache-returned-credentials-on-a-per-serviceaccount-basis-use-this-when-the)
- [returned credential is valid for all pods using the same service account.](#returned-credential-is-valid-for-all-pods-using-the-same-service-account)
- [requireServiceAccount indicates whether the plugin requires the pod to have a service account.](#requireserviceaccount-indicates-whether-the-plugin-requires-the-pod-to-have-a-service-account)
- [If set to true, kubelet will only invoke the plugin if the pod has a service account.](#if-set-to-true-kubelet-will-only-invoke-the-plugin-if-the-pod-has-a-service-account)
- [If set to false, kubelet will invoke the plugin even if the pod does not have a service account](#if-set-to-false-kubelet-will-invoke-the-plugin-even-if-the-pod-does-not-have-a-service-account)
- [and will not include a token in the CredentialProviderRequest. This is useful for plugins](#and-will-not-include-a-token-in-the-credentialproviderrequest-this-is-useful-for-plugins)
- [that are used to pull images for pods without service accounts (e.g., static pods).](#that-are-used-to-pull-images-for-pods-without-service-accounts-eg-static-pods)
- [requiredServiceAccountAnnotationKeys is the list of annotation keys that the plugin is interested in](#requiredserviceaccountannotationkeys-is-the-list-of-annotation-keys-that-the-plugin-is-interested-in)
- [and that are required to be present in the service account.](#and-that-are-required-to-be-present-in-the-service-account)
- [The keys defined in this list will be extracted from the corresponding service account and passed](#the-keys-defined-in-this-list-will-be-extracted-from-the-corresponding-service-account-and-passed)
- [to the plugin as part of the CredentialProviderRequest. If any of the keys defined in this list](#to-the-plugin-as-part-of-the-credentialproviderrequest-if-any-of-the-keys-defined-in-this-list)