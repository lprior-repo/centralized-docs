---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#6-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 505
summary: - [The keys defined in this list will be extracted from the corresponding service account and...
---

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