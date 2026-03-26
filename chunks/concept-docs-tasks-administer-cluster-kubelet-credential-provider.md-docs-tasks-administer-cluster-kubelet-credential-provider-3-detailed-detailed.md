---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 950
summary: - [kubelet will cache returned credentials on a per-serviceaccount basis. Use this when the](#kubelet-will-cache-returned-credentials-on-a-per-serviceaccount-basis-use-this-when-the) - [returned...
---

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
- [are not present in the service account, kubelet will not invoke the plugin and will return an error.](#are-not-present-in-the-service-account-kubelet-will-not-invoke-the-plugin-and-will-return-an-error)
- [This field is optional and may be empty. Plugins may use this field to extract additional information](#this-field-is-optional-and-may-be-empty-plugins-may-use-this-field-to-extract-additional-information)
- [required to fetch credentials or allow workloads to opt in to using service account tokens for image pull.](#required-to-fetch-credentials-or-allow-workloads-to-opt-in-to-using-service-account-tokens-for-image-pull)
- [The keys defined in this list must be unique and not overlap with the keys defined in the](#the-keys-defined-in-this-list-must-be-unique-and-not-overlap-with-the-keys-defined-in-the)
- [optionalServiceAccountAnnotationKeys is the list of annotation keys that the plugin is interested in](#optionalserviceaccountannotationkeys-is-the-list-of-annotation-keys-that-the-plugin-is-interested-in)
- [and that are optional to be present in the service account.](#and-that-are-optional-to-be-present-in-the-service-account)
- [The keys defined in this list will be extracted from the corresponding service account and passed](#the-keys-defined-in-this-list-will-be-extracted-from-the-corresponding-service-account-and-passed)
- [to the plugin as part of the CredentialProviderRequest. The plugin is responsible for validating the](#to-the-plugin-as-part-of-the-credentialproviderrequest-the-plugin-is-responsible-for-validating-the)
- [existence of annotations and their values. This field is optional and may be empty.](#existence-of-annotations-and-their-values-this-field-is-optional-and-may-be-empty)
- [Plugins may use this field to extract additional information required to fetch credentials.](#plugins-may-use-this-field-to-extract-additional-information-required-to-fetch-credentials)
- [The keys defined in this list must be unique and not overlap with the keys defined in the](#the-keys-defined-in-this-list-must-be-unique-and-not-overlap-with-the-keys-defined-in-the)
- [+optional](#optional)
      - [Configure image matching](#configure-image-matching)
  - [Feedback](#feedback)

---